package main

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"strings"
)

type serverResult struct {
	content []byte
	err     error
}

type applicationService struct {
	serverAddress string
}

func (r *applicationService) GetApplicationData(key string, ctx context.Context) ([]byte, error) {

	responseChannel := make(chan *serverResult)

	go func() {

		result := serverResult{
			content: nil,
			err:     nil,
		}

		defer close(responseChannel)

		response, err := http.Get(fmt.Sprintf("http://%s/serve/%s", r.serverAddress, key))

		if err != nil {
			result.err = err
			responseChannel <- &result
			return
		}

		defer response.Body.Close()

		contentType := response.Header.Get("Content-type")
		if !strings.Contains(contentType, "application/json") {
			result.err = fmt.Errorf("invalid content type %s", contentType)
			responseChannel <- &result
			return
		}

		body, err := io.ReadAll(response.Body)

		if err != nil {
			result.err = err
			responseChannel <- &result
			return
		}

		result.content = body
		result.err = nil

		responseChannel <- &result
	}()

	select {
	case <-ctx.Done():
		return nil, fmt.Errorf("timed out waiting for request to finish")
	case result := <-responseChannel:
		if result.err != nil {
			fmt.Println(result.err.Error())
		}
		return result.content, result.err
	}
}
