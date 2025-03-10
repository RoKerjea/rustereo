
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
	let mut vec1 = vec![];
	vec1.push((1.0,1.0));
	vec1.push((1.0,0.0));
	vec1.push((0.0,0.0));
	vec1.push((0.0,1.0));
	let poly1 = Polygon::new(vec1);
	let mut vec2 = vec![];
	// vec2.push((3.0,3.0));
	// vec2.push((3.0,2.0));
	// vec2.push((2.0,2.0));
	// vec2.push((2.0,3.0));
	vec2.push((0.5,0.5));
	vec2.push((0.5,1.5));
	vec2.push((1.5,1.5));
	vec2.push((1.5,0.5));
	let poly2 = Polygon::new(vec2);
	if polygon_collision(&poly1, &poly2){
		eprintln!("they collide");
	} else {
		eprintln!("they don't collide");
	}
}
pub struct Polygon
{
	points: Vec<(f32, f32)>,
	edges: Vec<(f32, f32)>,
}

impl Polygon
{
	pub fn new(points_given: Vec <(f32, f32)>) -> Self
	{
		let mut new_edges : Vec<(f32, f32)> = vec![];
		for i in 0..points_given.len() - 1
		{
			let edge = (points_given[i + 1].0 - points_given[i].0, points_given[i + 1].1 - points_given[i].1);
			new_edges.push(edge);
		}
		let edge = (points_given[points_given.len() - 1].0 - points_given[0].0, points_given[points_given.len() - 1].1 - points_given[0].1);
			new_edges.push(edge);
		Self {
			points: points_given,
			edges: new_edges,
		}
	}
}

// fn perpendicula_axis(edge: (f32, f32)) -> (f32, f32)
// {
// 	return (-edge.1, edge.0);
// }

fn dot_product(axis: (f32,f32), point: (f32,f32)) -> f32
{
	return axis.0 * point.0 + axis.1 * point.1;
}

fn project_polygon(axis: (f32,f32), polygon : &Polygon) -> (f32,f32)
{
	let mut dot_prod = dot_product(axis, polygon.points[0]);
	let mut min = dot_prod;
	let mut max = dot_prod;
	for i in 0..polygon.points.len()
	{
		dot_prod = dot_product(axis, polygon.points[i]);
		if dot_prod< min {
			min = dot_prod;
		} else if dot_prod > max {
			max = dot_prod;
		}
	}
	return (min, max);
}

fn interval_distance(minmax_a: (f32,f32), minmax_b: (f32,f32)) -> f32
{
	if minmax_a.0 < minmax_b.0{
		return minmax_b.0 - minmax_a.1;
	}
	else {
		return minmax_a.0 - minmax_b.1;
	}
}

fn polygon_collision(poly1: &Polygon, poly2: &Polygon) -> bool
{
	let edges1_count = poly1.edges.len();
	let edges2_count = poly2.edges.len();
	let edges_sum = edges1_count + edges2_count;

	let mut edge: (f32, f32);
	for i in 0..edges_sum
	{
		if i < edges1_count
		{
			edge = poly1.edges[i];
		} else {
			edge = poly2.edges[i - edges1_count];
		}
		let axis  =  (-edge.1,edge.0);//need normalized axis
		//create 4 float for min and max x and y of both polygons
		let minmax_a = project_polygon(axis, poly1);
		let minmax_b = project_polygon(axis, poly2);
		if interval_distance(minmax_a, minmax_b) < 0.0
		{
			return true;
		}
		//check intervaldistance from thos 4 points
		//can add a willintersect check, which could probably be used to make sure those 2 polygons are at least at a certain distance of each others
		//end of loop
	}
	false
}
