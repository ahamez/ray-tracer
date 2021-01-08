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

![Cover](/samples/render/cover.png?raw=true "Cover")

![Shadow Glamour Shot](/samples/render/shadow-glamour-shot.png?raw=true "Shadow Glamour Shot")

![Plane](/samples/render/ch09_plane.png?raw=true "Plane")

![Pattern](/samples/render/ch10_pattern.png?raw=true "Pattern")

![Reflection](/samples/render/ch11_reflection.png?raw=true "Reflection")

![Refraction](/samples/render/ch11_refraction.png?raw=true "Refraction")

![Fresnel](/samples/render/ch11_fresnel.png?raw=true "Fresnel")

![Reflect-Refract](/samples/render/ch11_reflect-refract.png?raw=true "Reflect Refract")

![Cube](/samples/render/ch12_cube.png?raw=true "Cube")

![Cylinder](/samples/render/ch13_cylinder.png?raw=true "Cylinder")

![Cone](/samples/render/ch13_cone.png?raw=true "Cone")

![Hexagon](/samples/render/ch14_hexagon.png?raw=true "Hexagon")

![Triangle](/samples/render/ch15_triangle.png?raw=true "Triangle")

![Lunar lander](/samples/render/lunar_lander.png?raw=true "Lunar Lander")

![Armadillo](/samples/render/armadillo_small.png?raw=true "Armadillo")

![Astronaut](/samples/render/astronaut.png?raw=true "Astronaut")
