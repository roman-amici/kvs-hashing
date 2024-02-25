package main

import (
	"fmt"
	"net/http"

	"github.com/gorilla/mux"
)

func main() {
	router := mux.NewRouter()

	cache := NewCache()
	service := applicationService{
		serverAddress: "localhost:5177",
	}

	router.HandleFunc("/serve/{key:.*}", func(w http.ResponseWriter, r *http.Request) {
		handleCachedRequest(w, r, &cache, &service)
	}).Methods("GET")

	http.Handle("/", router)

	fmt.Println("Starting server on :4001")
	err := http.ListenAndServe(":4001", nil)
	if err != nil {
		println(err)
	}
}

func handleCachedRequest(w http.ResponseWriter, r *http.Request, c *cache, service *applicationService) {
	params := mux.Vars(r)
	key := params["key"]

	if value, err := c.Get(key); err == nil {
		w.Header().Set("Content-Type", "application/json")
		w.Write(value)
		return
	}

	if value, err := service.GetApplicationData(key, r.Context()); err == nil {
		c.Set(key, value)

		w.Header().Set("Content-Type", "application/json")
		w.Write(value)
		return
	}

	w.WriteHeader(404)
}
