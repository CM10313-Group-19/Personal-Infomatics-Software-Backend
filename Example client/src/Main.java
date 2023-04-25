import okhttp3.*;

import java.io.IOException;
import java.util.Dictionary;
import java.util.Enumeration;
import java.util.Hashtable;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

public class Main {

    private static final String url = "http://188.166.144.209:8000";

    public static void main(String[] args) throws IOException {
        System.out.println(check_email_in_use("email2@email.com"));
        System.out.println(signup_user("email2@email.com", "password", "2000-1-1"));
        System.out.println(login_user("email2@email.com", "password"));
    }

    /**
     * Checks if an email exists
     * @param email Email to check
     * @return true if email exists, false if not
     * @throws IOException exception thrown if request failed
     */
    public static Boolean check_email_in_use(String email) throws IOException {
        // Create parameters dictionary
        Dictionary<String, String> parameters = new Hashtable<String, String>();
        parameters.put("email", email);
        var response = get_request(url, "/check_email", parameters);

        // Check if the request was successful
        if (response.isSuccessful()) {
            // Parse the response body into json
            var res = parse_json(response.body().string());
            // Get the `exists` variable from the json response
            return res.get("exists").asBoolean();
        } else {
            // If the response failed return null
            throw new IOException("Request was not successful");
        }
    }

    /**
     * Adds a new user
     * @param email         Email of the new user
     * @param password      password of the new user
     * @param date_of_birth Date of birth of the new user, in the form `Y-m-d`
     * @return Returns true or false depending on if the user was created successfully
     */
    public static Boolean signup_user(String email, String password, String date_of_birth) {
        // Create the form data
        Dictionary<String, String> form_data = new Hashtable<String, String>();
        form_data.put("email", email);
        form_data.put("password", password);
        form_data.put("date_of_birth", date_of_birth);

        try {
            // Attempt the post request
            var response = post_request("http://188.166.144.209:8000", "/signup", form_data);

            // Check it returned successfully
            if (response.isSuccessful()) {
                // Parse to json
                var json = parse_json(response.body().string());

                // Return the value of the `success field`, there is also a `message` field which gives error messages
                return json.get("success").asBoolean();
            } else return false;
        } catch (Exception e) {
            return false;
        }
    }

    /**
     * Logs in a user
     * @param email Email of the user
     * @param password Password of the user
     * @return If the login was successful the user_id of the user, otherwise -1
     */
    public static int login_user(String email, String password) {
        Dictionary<String, String> parameters = new Hashtable<>();
        parameters.put("email", email);
        parameters.put("password", password);

        try {
            var response = get_request(url, "/login", parameters);

            if(response.isSuccessful()) {
                var json = parse_json(response.body().string());

                return json.get("user_id").asInt();
            } else return -1;
        } catch (Exception e) {
            return -1;
        }
    }

    public static JsonNode parse_json(String json) {
        try {
            ObjectMapper objectMapper = new ObjectMapper();
            return objectMapper.readTree(json);
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }
    }

    public static Response post_request(String base_url, String sub_dir, Dictionary<String, String> form) throws IOException {
        HttpUrl.Builder urlBuilder = HttpUrl.parse(base_url + sub_dir).newBuilder();

        // Create a new form and add the values from the passed in dictionary
        var body = new FormBody.Builder();
        for (Enumeration<String> enm = form.keys(); enm.hasMoreElements(); ) {
            var key = enm.nextElement();
            body.add(key, form.get(key));
        }

        // Create url and request
        String url = urlBuilder.build().toString();
        var request = new Request.Builder().url(url).post(body.build()).build();

        // Execute the request
        Call call = new OkHttpClient().newCall(request);

        // Return the response of the request
        return call.execute();
    }

    public static Response get_request(String base_url, String sub_dir, Dictionary<String, String> parameters) throws IOException {
        HttpUrl.Builder urlBuilder = HttpUrl.parse(base_url + sub_dir).newBuilder();

        // Iterate over the parameters and add them as query parameters
        for (Enumeration<String> enm = parameters.keys(); enm.hasMoreElements(); ) {
            var key = enm.nextElement();
            urlBuilder.addQueryParameter(key, parameters.get(key));
        }

        // Create url and request
        String url = urlBuilder.build().toString();
        var request = new Request.Builder().url(url).get().build();

        // Execute the request
        Call call = new OkHttpClient().newCall(request);

        return call.execute();
    }
}
