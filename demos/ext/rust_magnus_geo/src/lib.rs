use magnus::{
    self as magnus, define_class, function, gc, method, prelude::*,
    DataTypeFunctions, Error, TypedData, Value,
};

// Create a new struct for magnus to wrap with `rb_data_typed_object_wrap`. This
// will setup the boiler plate for making this struct accessible to Ruby.
#[derive(Debug)]
#[magnus::wrap(class = "Point")]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    /// Create a new `Point` with the given `x` and `y` values.
    ///
    /// Did you know: Comments with `///` will be treated as documenation,
    /// so you can hover over this method and see this cool text!
    ///
    /// Comments with `//` will not be treated as documentation.
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Get the `x` value of this `Point`.
    fn x(&self) -> isize {
        self.x
    }

    /// Get the `y` value of this `Point`.
    fn y(&self) -> isize {
        self.y
    }

    /// Calculate the distance between this point and another point.
    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }

    /// Calculate the dot product between two points.
    fn dot(&self, other: &Point) -> isize {
        self.x * other.x + self.y * other.y
    }
}

/// Represents a rectangle, containing two `Point`s.
#[derive(Debug, TypedData)]
#[magnus(class = "Rectangle", mark)]
struct Rectangle {
    top_left: Value,
    bottom_right: Value,
}

// We implement the `DataTypeFunctions` trait so we can implement the `mark`,
// and ensure the `top_left` and `bottom_right` values do not get GC'd while we
// are using them.
impl DataTypeFunctions for Rectangle {
    fn mark(&self) {
        gc::mark(&self.top_left);
        gc::mark(&self.bottom_right);
    }
}

impl Rectangle {
    /// Create a new `Rectangle` with the given `top_left` and `bottom_right`
    fn new(top_left: Value, bottom_right: Value) -> Result<Self, Error> {
        Ok(Self {
            top_left,
            bottom_right,
        })
    }

    /// Get the top left `Point` of this `Rectangle`.
    fn top_left(&self) -> Value {
        self.top_left
    }

    /// Get the bottom right point of this rectangle.
    fn bottom_right(&self) -> Value {
        self.bottom_right
    }

    /// Checks if the given `Point` is inside this `Rectangle`.
    fn contains(&self, other: &Point) -> Result<bool, Error> {
        /*
        let top_left = self.top_left.try_convert::<&Point>()?;
        let bottom_right = self.bottom_right.try_convert::<&Point>()?;

        let contains_x = top_left.x <= other.x && other.x <= bottom_right.x;
        let contains_y = top_left.y >= other.y && other.y >= bottom_right.y;
        Ok(contains_x && contains_y)
        */

        // A and C are two corners of the rectangle.
        let a = self.top_left.try_convert::<&Point>()?;
        let c = self.bottom_right.try_convert::<&Point>()?;

        // We assume the rectangle is regular, so we can calculate one of the other corners.
        let b = Point::new(c.x, a.y);

        // We can now calculate whether the point is inside the rectangle by the following formula:
        //   0 <= dot(AB, AM) <= dot(AB, AB) &&
        //   0 <= dot(BC, BM) <= dot(BC, BC)
        // where M is the point we are checking, and A, B, and C are the corners of the rectangle.
        let ab = Point::new(b.x - a.x, b.y - a.y);
        let bc = Point::new(c.x - b.x, c.y - b.y);
        let am = Point::new(other.x - a.x, other.y - a.y);
        let bm = Point::new(other.x - b.x, other.y - b.y);

        let abam = ab.dot(&am);
        let abab = ab.dot(&ab);
        let bcbm = bc.dot(&bm);
        let bcbc = bc.dot(&bc);

        Ok(0 <= abam && abam <= abab && 0 <= bcbm && bcbm <= bcbc)
    }
}

// Mark as thread-safe for purposes of the demo
unsafe impl Send for Rectangle {}

#[magnus::init]
fn init() -> Result<(), Error> {
    let point_class = define_class("Point", Default::default())?;
    point_class.define_singleton_method("new", function!(Point::new, 2))?;
    point_class.define_method("x", method!(Point::x, 0))?;
    point_class.define_method("y", method!(Point::y, 0))?;
    point_class.define_method("distance", method!(Point::distance, 1))?;
    point_class.define_method("dot", method!(Point::distance, 1))?;

    let rectangle_class = define_class("Rectangle", Default::default())?;
    rectangle_class.define_singleton_method("new", function!(Rectangle::new, 2))?;
    rectangle_class.define_method("top_left", method!(Rectangle::top_left, 0))?;
    rectangle_class.define_method("bottom_right", method!(Rectangle::bottom_right, 0))?;
    rectangle_class.define_method("contains?", method!(Rectangle::contains, 1))?;
    Ok(())
}
