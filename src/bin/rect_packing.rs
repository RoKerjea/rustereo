
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
/*rect generation:
nannou can use size and a point to generate a rect
but i can also use lines to create any polygons i want
lines might be closer to the polygon detection function, but a bit more complex for a start
while 0->100
rand corner value between xmin and xmax, same for y
size of 50 at start
to detect, i need to create a polygon struct
so, a vec of points, corner value, coner value + xsize, plus ysize, plus both
call new polygon on it

debug idea:
print list of polygons at end of loop to see wich are colliding to debug
*/
use rand::Rng;
use nannou::prelude::*;
use nannou::event::Key;
// fn main()
// {
// 	let mut vec1 = vec![];
// 	vec1.push(vec2(597.9022,94.52768));
// 	vec1.push(vec2(597.9022,124.52768));
// 	vec1.push(vec2(627.9022,124.52768));
// 	vec1.push(vec2(627.9022,94.52768));
// 	let poly1 = Polygon::new(&vec1, 1);
// 	let mut vecy = vec![];
// 	vecy.push(vec2(603.067, 85.802216));
// 	vecy.push(vec2(603.067,115.802216));
// 	vecy.push(vec2(633.067,115.802216));
// 	vecy.push(vec2(633.067,85.802216));
// 	let poly2 = Polygon::new(&vecy, 2);
// 	if polygon_collision(&poly1, &poly2){
// 		eprintln!("they collide");
// 	} else {
// 		eprintln!("they don't collide");
// 	}
// }
// id : 12
// p1 : 597.9022 94.52768
// p2 : 597.9022 124.52768
// p3 : 627.9022 124.52768
// p4 : 627.9022 94.52768
// id : 13
// p1 : 603.067 85.802216
// p2 : 603.067 115.802216
// p3 : 633.067 115.802216
// p4 : 633.067 85.802216
fn main() {
	nannou::app(model)
	.loop_mode(LoopMode::Wait)
	.update(update)
	.run();
}

struct Model {
	canvas: nannou::draw::Draw,
	height: u32,
	width: u32,
}

fn model(app: &App) -> Model {

	let height = 720;
	let width = 1280;
	let _w_id = app
		.new_window()
		.size(width, height)
		.title("rect")
		.event(event)
		.view(view)
		.build()
		.unwrap();
	let canvas = rect_packing(height, width);
	Model {
		canvas,
		height,
		width,
	}
}

fn random_color2() -> nannou::color::Rgba {
	let color1 = nannou::color::Rgba::new(96.0 / 255.0, 108.0 / 255.0, 56.0 / 255.0, 1.0);
	let color2 = nannou::color::Rgba::new(40.0 / 255.0, 54.0 / 255.0, 24.0 / 255.0, 1.0);
	let color3 = nannou::color::Rgba::new(254.0 / 255.0, 250.0 / 255.0, 224.0 / 255.0, 1.0);
	let color4 = nannou::color::Rgba::new(221.0 / 255.0, 161.0 / 255.0, 94.0 / 255.0, 1.0);
	let color5 = nannou::color::Rgba::new(188.0 / 255.0, 108.0 / 255.0, 37.0 / 255.0, 1.0);
	let mut rng = rand::thread_rng();
	let color = rng.gen_range(0..=4);
	match color {
		0 => color1,
		1 => color2,
		2 => color3,
		3 => color4,
		4 => color5,
		_ => color1,
	}
}

fn rect_packing(height:u32, width:u32) -> nannou::draw::Draw
{
	let canvas = nannou::draw::Draw::new();
	canvas.background().color(WHITE);
	let color = random_color2();
	let mut rng = rand::thread_rng();
	// let y = rng.gen_range(0.0..=height as f32) - height as f32/2.0;
	// let x = rng.gen_range(0.0..=width as f32) - width as f32/2.0;
	// let mut vec1 = vec![];
	// vec1.push((x, y));
	// vec1.push((x+30.0, y));
	// vec1.push((x+30.0, y+30.0));
	// vec1.push((x, y+30.0));
	// let poly1 = Polygon::new(vec1);
	// let mut polygon_list = vec![];
	// polygon_list.push(poly1);
	// canvas.rect()
	// 	.color(color)
	// 	.w_h(30.0, 30.0)
	// 	.x_y(x + 15.0, y + 15.0);
	// let n_points = 4;
    // let radius = 30.0;
    // let points = (0..n_points).map(|i| {
    //     let fract = i as f32 / n_points as f32;
    //     let phase = fract;
    //     let x = radius * (TAU * phase).cos();
    //     let y = radius * (TAU * phase).sin();
    //     pt2(x, y)
    // });
	let mut size = 100.0;
	let mut vec = vec![];
	let x = 0.0;
	let y = 0.0;
	vec.push(pt2(x, y));
	vec.push(pt2(x, y+size));
	vec.push(pt2(x+size, y+size));
	vec.push(pt2(x+size, y));
	let poly1 = Polygon::new(&vec, 0);
    canvas.polygon()
        .color(color)
        .stroke(BLACK)
        .stroke_weight(2.0)
        .join_round()
        .points(vec);
	let mut polygon_list = vec![];
	polygon_list.push(poly1);
	for _i in 0..50{
		let color = random_color2();
		let y = rng.gen_range(0.0..=height as f32 - size) - height as f32/2.0;
		let x = rng.gen_range(0.0..=width as f32 - size) - width as f32/2.0;
		let mut vec = vec![];
		vec.push(pt2(x, y));
		vec.push(pt2(x, y+size));
		vec.push(pt2(x+size, y+size));
		vec.push(pt2(x+size, y));
		let poly = Polygon::new(&vec, _i);
		if !new_poly_collision(&poly, &polygon_list)
		{
			polygon_list.push(poly);
			canvas.polygon()
				.color(color)
				.stroke(BLACK)
				.stroke_weight(2.0)
				.join_round()
				.points(vec);
		}
	}
	size = 30.0;
	for _i in 0..8000{
		let color = random_color2();
		let y = rng.gen_range(0.0..=height as f32 - size) - height as f32/2.0;
		let x = rng.gen_range(0.0..=width as f32 - size) - width as f32/2.0;
		let mut vec = vec![];
		vec.push(pt2(x, y));
		vec.push(pt2(x, y+size));
		vec.push(pt2(x+size, y+size));
		vec.push(pt2(x+size, y));
		let poly = Polygon::new(&vec, _i);
		if !new_poly_collision(&poly, &polygon_list)
		{
			polygon_list.push(poly);
			canvas.polygon()
				.color(color)
				.stroke(BLACK)
				.stroke_weight(2.0)
				.join_round()
				.points(vec);
		}
	}
	size = 10.0;
	for _i in 0..7000{
		let color = random_color2();
		let y = rng.gen_range(0.0..=height as f32 - size) - height as f32/2.0;
		let x = rng.gen_range(0.0..=width as f32 -size) - width as f32/2.0;
		let mut vec = vec![];
		vec.push(pt2(x, y));
		vec.push(pt2(x, y+size));
		vec.push(pt2(x+size, y+size));
		vec.push(pt2(x+size, y));
		let poly = Polygon::new(&vec, _i);
		if !new_poly_collision(&poly, &polygon_list)
		{
			polygon_list.push(poly);
			canvas.polygon()
				.color(color)
				.stroke(BLACK)
				.stroke_weight(2.0)
				.join_round()
				.points(vec);
		}
	}
	eprintln!("there are {} polygons", polygon_list.len());
	// for i in 0..polygon_list.len() - 1 {
	// 	polygon_list[i].print_poly_data();
	// }
	canvas
}

fn new_poly_collision(new_poly: &Polygon, poly_list: &Vec<Polygon>) -> bool
{
	for i in 0..poly_list.len(){
		if polygon_collision(new_poly, &poly_list[i]){
			return true
		}
	}
	false
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
	match event {
		WindowEvent::KeyReleased(key) => {
			println!("{:?}", key);
			match key {
				Key::Space => {
					_model.canvas = rect_packing(_model.height, _model.width);
				}
				_ => {}
			}
		}
		_ => {}
	}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) {
	_model.canvas.to_frame(_app, &frame).unwrap();
	let file_path = "assets/images/rect.png";
    _app.main_window().capture_frame(file_path);
}
pub struct Polygon
{
	id: u32,
	points: Vec<Vec2>,
	edges: Vec<Vec2>,
}

impl Polygon
{
	pub fn new(points_given: &Vec <Vec2>, _id:u32) -> Self
	{
		let mut new_edges : Vec<Vec2> = vec![];
		for i in 0..points_given.len() - 1
		{
			let edge = points_given[i + 1] - points_given[i];
			new_edges.push(edge);
		}
		let edge = points_given[0] - points_given[points_given.len() - 1];
			new_edges.push(edge);
		Self {
			id:_id,
			points: points_given.to_vec(),
			edges: new_edges,
		}
	}
	pub fn print_poly_data(&self)
	{
		eprintln!("id : {}", self.id);
		eprintln!("p1 : {} {}", self.points[0][0], self.points[0][1]);
		eprintln!("p2 : {} {}", self.points[1][0], self.points[1][1]);
		eprintln!("p3 : {} {}", self.points[2][0], self.points[2][1]);
		eprintln!("p4 : {} {}", self.points[3][0], self.points[3][1]);
	}
}

fn perpendicular_axis(edge: Vec2) -> Vec2
{
	let axis = Vec2::new(-edge.y, edge.x);
	axis.normalize()
}

fn project_polygon(axis: Vec2, polygon : &Polygon) -> (f32,f32)
{
	let mut dot_prod = axis.dot(polygon.points[0]);
	let mut min = dot_prod;
	let mut max = dot_prod;
	for i in 0..polygon.points.len()
	{
		dot_prod = polygon.points[i].dot(axis);
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
	let mut result = true;

	let mut edge: Vec2;
	for i in 0..edges_sum
	{
		if i < edges1_count
		{
			edge = poly1.edges[i];
		} else {
			edge = poly2.edges[i - edges1_count];
		}
		let axis  = perpendicular_axis(edge);//need normalized axis
		//create 4 float for min and max x and y of both polygons
		let mut minmax_a = project_polygon(axis, poly1);
		let minmax_b = project_polygon(axis, poly2);
		minmax_a.0 -= 4.0;
		minmax_a.1 += 4.0;
		if interval_distance(minmax_a, minmax_b) > 0.0
		{
			// eprintln!("found a separating axis for points min A: {} , max A: {}", minmax_a.0, minmax_a.1);
			// eprintln!("min B: {} , max B: {}", minmax_b.0, minmax_b.1);
			// eprintln!("axis: {} {}", axis[0], axis[1]);
			return false;
		}
		if poly1.points[0][0] > poly2.points[0][0] + 4.0 && poly1.points[0][1] > poly2.points[0][1] + 4.0 && poly1.points[2][0] < poly2.points[2][0] -4.0 && poly1.points[2][1] < poly2.points[2][1] -4.0
		{
			return false;
		}
		//check intervaldistance from thos 4 points
		//can add a willintersect check, which could probably be used to make sure those 2 polygons are at least at a certain distance of each others
		//end of loop
	}
	result
}
