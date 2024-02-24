using System.Text.Json;

namespace app_server.Services;

public class FileContentService : IContentService
{
    public JsonElement GetDocument(string key)
    {
        ReadOnlySpan<byte> data = File.ReadAllBytes(Path.Combine("content", key));
        var reader = new Utf8JsonReader(data);

        return JsonElement.ParseValue(ref reader);
    }
}