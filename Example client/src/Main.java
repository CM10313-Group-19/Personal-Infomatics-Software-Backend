import okhttp3.*;
import java.io.IOException;
import java.util.Dictionary;
import java.util.Enumeration;
import java.util.Hashtable;

public class Main {
    public static void main(String[] args) throws IOException {
        Dictionary<String, String> parameters = new Hashtable<String, String>();
        parameters.put("email", "email@email.com");
        System.out.println(get_request("http://127.0.0.1:8000", "/check_email", parameters));

        Dictionary<String, String> new_user = new Hashtable<String, String>();
        new_user.put("email", "email@email.com");
        new_user.put("password", "password");
        new_user.put("date_of_birth", "2000-1-1");
        System.out.println(post_request("http://127.0.0.1:8000", "/signup", new_user));

        // Data is currently returned in json form, Next step is to add a json parser to convert the data returned into
        // java classes
    }

    public static String post_request(String base_url, String sub_dir, Dictionary<String, String> form) throws IOException {
        HttpUrl.Builder urlBuilder
                = HttpUrl.parse(base_url + sub_dir).newBuilder();

        // Create a new form and add the values from the passed in dictionary
        var body = new FormBody.Builder();
        for(Enumeration<String> enm = form.keys(); enm.hasMoreElements();)
        {
            var key = enm.nextElement();
            body.add(key, form.get(key));
        }

        // Create url and request
        String url = urlBuilder.build().toString();
        var request = new Request.Builder()
                .url(url).post(body.build()).build();

        // Execute the request
        Call call = new OkHttpClient().newCall(request);
        Response response = call.execute();

        // Return the body of the request
        // TODO: check that the request was a success
        return response.body().string();
    }

    public static String get_request(String base_url, String sub_dir, Dictionary<String, String> parameters) throws IOException {
        HttpUrl.Builder urlBuilder
                = HttpUrl.parse(base_url + sub_dir).newBuilder();

        // Iterate over the parameters and add them as query parameters
        for(Enumeration<String> enm = parameters.keys(); enm.hasMoreElements();)
        {
            var key = enm.nextElement();
            urlBuilder.addQueryParameter(key, parameters.get(key));
        }

        // Create url and request
        String url = urlBuilder.build().toString();
        var request = new Request.Builder()
                .url(url).get().build();

        // Execute the request
        Call call = new OkHttpClient().newCall(request);
        Response response = call.execute();

        return response.body().string();
    }
}