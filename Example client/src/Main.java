import java.io.Console;
import java.io.IOException;
import java.io.InputStream;
import java.net.HttpURLConnection;
import java.net.URI;
import java.net.URL;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

class checkEmail {
    String email;
}

public class Main {
    public static void main(String[] args) throws IOException, InterruptedException {
        // create a client
        var client = HttpClient.newHttpClient();

        var data = new checkEmail();
        data.email = "test@email.com";

        // create a request
        var request = HttpRequest.newBuilder(
                        URI.create("http://127.0.0.1:8000/user"))
                .header("accept", "application/json")
                .build();

        // use the client to send the request
        var response = client.send(request,
                HttpResponse.BodyHandlers.ofString());
        System.out.println(response.body());
    }
}