using System.Text;
using System.Text.Json;
using app_server.Services;
using Microsoft.AspNetCore.Mvc;

namespace app_server.Controllers;

[ApiController]
[Route("[controller]")]
public class ServeController : ControllerBase
{
    private IContentService _contentService;

    public ServeController(IContentService contentService)
    {
        _contentService = contentService;
    }

    [HttpGet("{*path}")]
    public JsonElement Get(string path)
    {
        return _contentService.GetDocument(path);
    }
}
