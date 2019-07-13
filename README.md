   # Image Loader REST API
   Description...
   
   ## Dependencies
   * Rust 1.36
   * Rocket 0.4.2
   * Docker 18.09.3
      
   ## Installation and Run
   Copy project:
   ```
   $ git clone https://github.com/backuot/imageloader.git
   $ cd imageloader
   ```

   Use Docker:
   ```
   $ docker-compose up
   ```

   Or use Cargo:
   ```
   $ cargo run
   $ cargo test
   ```

   ## Usage
   The result of the following queries is an HTML page with images. You must use the image URL in the request.
   
   For download image:
   ```
   $ curl -i "http://localhost:8000/load?url=http://example.com/image1.png"
   ```

   For download more than one image use ";" separator in query:
   ```
   $ curl -i "http://localhost:8000/load?url=http://example.com/image1.png;http://example.com/image2.png"
   ```

   ## Versions
   * 0.0.1
   
  ## Authors
  * **Dmitry Nazarov**
  
  ## License
  This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.