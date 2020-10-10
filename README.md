# Ray Tracing with Rust
Rust implementation of the code found in the Ray Tracing series by Peter Shirley. I am using the code of <a href="https://github.com/fralken">fralken</a> and <a href="https://github.com/cbiffle/rtiow-rust">cbiffle</a> as a guide for writing idiomatic Rust.

<p align="center">
  <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/test.jpg" />
</p>


## Progress

### Extra
<details><summary>Concurrency</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter13.jpg" />
  </p>
  <p>Concurrency with Rayon. Reduced final image render time for 60+ minutes to 8 minutes.</p>
</details>


### Book 2
<details><summary>Chapter 2</summary>
    <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/book2_chapter3.1.jpg" />
  </p>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/book2_chapter3.2.jpg" />
  </p>
  <p>Ground work for allowing real textures.</p>
</details>

<details><summary>Chapter 2</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/book2_chapter2.jpg" />
  </p>
  <p>BVH acceleration structure. Reduces render time from 9 minutes to 3 minutes. 200% speed increase. </p>
</details>

<details><summary>Chapter 1</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/book2_chapter1.jpg" />
  </p>
  <p>Motion blur.</p>
</details>


### Book 1
<details><summary>Chapter 12</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter12.jpg" />
  </p>
  <p>Rendering a randomised scene.</p>
</details>

<details><summary>Chapter 11</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter11.jpg" />
  </p>
  <p>Depth of field added.</p>
</details>

<details><summary>Chapter 10</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter10.jpg" />
  </p>
  <p>Camera struct extended. Provide a point to look from, point to look at, up direction, vertical FOV and aspect ratio.</p>
</details>

<details><summary>Chapter 9</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter9.jpg" />
  </p>
  <p>Dielectric material struct created.</p>
</details>

<details><summary>Chapter 8</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter8.jpg" />
  </p>
  <p>Material trait added and material structs (lambertian and metal) created.</p>
</details>


<details><summary>Chapter 7</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter7full.jpg" />
  </p>
  <p>Lambertian reflection added, Uniform Distribution used for randomness to improve speed.</p>
</details>

<details><summary>Chapter 6</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter6.jpg" />
  </p>
  <p>Camera struct created and multiple passes added to main loop for anti-aliasing.</p>
</details>

<details><summary>Chapter 5</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter5.jpg" />
  </p>
  <p>Hittable Trait implemented. HitRecord and HittableList structs added.</p>
</details>

<details><summary>Chapter 4</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter4.jpg" />
  </p>
  <p>Added very basic shpere intersection.</p>
</details>


<details><summary>Chapter 3</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter3.jpg" />
  </p>
  <p>Improved Vec3 struct and created Ray struct. Added a ray_colour function.</p>
</details>


<details><summary>Chapter 2</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter2.jpg" />
  </p>
  <p>Vec3 and Colour structs implemented.</p>
</details>


<details><summary>Chapter 1</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Ray-Tracing-with-Rust/blob/master/img/chapter1.jpg" />
  </p>
  <p>Writing to a ppm file.</p>
</details>
