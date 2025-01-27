
/* tryng to do rectangle packing like steve makerspace
firt pb is : detecting is two rectangles intersect at their edges
(it's okay is a rectangle is contained in another one, but it's a bit too complex for now)
multiple formulas exists, but there is one which is extandable to all polygons, including polygons of different shapes from each other
(can also be extended to detect if moving polygons WILL intersect)
https://www.codeproject.com/Articles/15573/2D-Polygon-Collision-Detection

need:
a polygon struct containing:
an array of points(x, y), vertices
an array of edges, each edge is a vector from a point to another point
(if i keep the points, i don't need to apply rotation operations at any point)
which means i need to apply roation operations before checking for collision

steps to test:
create a rectangle from points+size (need rotation too)
compapre two rectangles
compare one rect to all existing ones
draw all those
paint inside of rect, or take part of another image toput in place of that one

 */


fn main()
{

}
pub struct Polygon
{
	points: Vec<(f32, f32)>,
	edges: Vec<(f32, f32)>,
}

fn polygon_collision(poly1: &Polygon, poly2: &Polygon) -> bool
{
	let edges1_count = poly1.edges.len();
	let edges2_count = poly2.edges.len();
	let edges_sum = edges1_count + edges2_count;

	for i in 0..edges_sum
	{
		if i < edges1_count
		{
			let edge = poly1.edges[i];
		} else {
			let edge = poly2.edges[i - edges1_count];
		}
		let axis = (-edge.1, edge.0);//need normalized axis
		//create 4 float for min and max x and y of both polygons
		//check intervaldistance from thos 4 points
		//end of loop
	}
	false
}
