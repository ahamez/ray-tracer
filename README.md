# Ray Tracer Challenge

This is an implementation in Rust of the book [The Ray Tracer Challenge](http://www.raytracerchallenge.com) . Go get it, it's one of the best book I read!

My main objective is to be fluent in Rust 🗣🦀. So, if some seasoned rustacean comes across this, don't hesitate to pinpoint the problems 😉!

## Differences from the book and other points of interest
* Parallelization with [rayon](https://github.com/rayon-rs/rayon) (I can't believe how easy it was!)
* Matrices inversion are cached (which provides a significant speedup!)
* Don't explicitly store the w component for tuples. Instead, it's up to the implementation of the Tuple trait to return the correct value (1.0 or 0.0)
* Avoid creating a new vec for each call to a shape intersects() method using a kind of closure (it provides an interesting speedup as it prevents the dynamic allocation of many temporaries)
* Use f64 everywhere

## Some samples

![Cover](/samples/cover.png?raw=true "Cover")

![Shadow Glamour Shot](/samples/shadow-glamour-shot.png?raw=true "Shadow Glamour Shot")

![Reflect-Refract](/samples/reflect-refract.png?raw=true "Reflect Refract")

![Fresnel](/samples/fresnel.png?raw=true "Fresnel")

![Hexagon](/samples/hexagon.png?raw=true "Hexagon")

![Plane](/samples/ch09_plane.png?raw=true "Plane")

![Pattern](/samples/ch10_pattern.png?raw=true "Pattern")

![Reflection](/samples/ch11_reflection.png?raw=true "Reflection")

![Refraction](/samples/ch11_refraction.png?raw=true "Refraction")

![Cube](/samples/ch12_cube.png?raw=true "Cube")

![Cylinder](/samples/ch13_cylinder.png?raw=true "Cylinder")

![Cone](/samples/ch13_cone.png?raw=true "Cone")

