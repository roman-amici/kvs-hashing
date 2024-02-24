using System.Text.Json;

namespace app_server.Services;

public interface IContentService 
{
    JsonElement GetDocument(string key);
}