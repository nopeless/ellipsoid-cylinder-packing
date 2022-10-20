
# Hexagonal packing of circles in arbitrary 3D shapes defined by an approximatable math function

In this case the 3D boundary is an ellipse, estimated by an 64 point contour

The code can be modified to work with any type of boundary, as long as there is a way to check if a circle collides with the boundary

The first circle of the hexagonal packing always starts at the edge of the boundary in hopes of maximizing the packing density. As shown by other papers, this type of approach is no where near the optimum

Nevertheless, this packing algorithm has the potential to be orders of magnitude faster than conventional packing algorithms by applying similar algorithms used by concave/convex hull algorithms--by rotating the circles at the edge by unit of 60 degrees and checking for collisions, then filling the inside. (willnot work for 3D objects with holes i.e. donuts. If this is the case, write an extra algorithm to loop these circles inside the donut as well)

Additionally, this hexagonal packing approach can easily extended to spheres, due to the fact that hexagonal packing of spheres is a well known and solved problem. Simple tweaks to the code should allow this to happen, however I would suggest implementing some sort of quad tree for collision detection as the 3D boundary becomes more complex

The current approach checks for collisions no matter where the circle is, which is not efficient

Currently it takes about 1.5 seconds to check for 444,000+ circles in a 3D ellipsoid, which I consider to be fast enough for most applications anyway

Credits to @Sniperzzzzz who provided a baseline number for the number of cylinders in a 3D ellipsoid

Hope you find this useful
