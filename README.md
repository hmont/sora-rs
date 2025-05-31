# sora-rs
Rust implementation of the [Sora](https://github.com/hmont/sora) image/file hosting server.

I do not recommend using this in production as it was one of my first ever Rust projects, is probably very buggy, and lacks many features expected of a hosting service. Nevertheless, it **should** "work" insofar as it fulfills the basic functions of a file hosting server (i.e. hosting images and files) but lacks many quality-of-life features that Sora has/will have (e.g. rate limiting, post deletion, any sort of frontend, etc.)

sora-rs requires Rust and a MongoDB database.

## Endpoints
- `/upload` - The endpoint used to upload files to the server. Takes two query parameters: an `api_key` and a `type` (either `"image"` or `"file"`) representing the user's API key and the type of file uploaded, respectively.
  - If you're using a ShareX client to upload (this is the intended use case), you can easily configure separate custom uploaders with different destination types (one as an image uploader and one as a file uploader) with different values for the `type` parameter. See the [ShareX Custom Uploader documentation](https://getsharex.com/docs/custom-uploader).
  - Returns a JSON response containing the `id` of the newly uploaded file, the file's `filename` (this is the same as the ID but includes the file extension), the username of the uploader (`uploader`), the `timestamp` when the file was uploaded, and the `type` of the file (same as the `type` parameter supplied in the request).
 
## Pages
- You can view a particular image/download a particular file by visiting the "endpoint" `/uploads/{id}` replacing `id` with the file's ID. Alternatively, if you know the file extension, you can visit `/files/{filename}` to view/download it without using the frontend.
- The index page (`/`) is just an informational page containing the number of files uploaded, number of images uploaded, and number of users in the database.
