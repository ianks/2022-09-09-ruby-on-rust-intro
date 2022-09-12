require "minitest/autorun"
require "demos/rust_magnus_geo"

class GeoTest < Minitest::Test
  def test_point_distance
    a = Point.new(0.0, 0.0)
    b = Point.new(0.0, 1.0)

    assert_equal(1.0, a.distance(b))
  end

  def test_rectangle_contains
    top_left = Point.new(0.0, 10.0)
    bottom_right = Point.new(10.0, 0.0)
    middle = Point.new(5.0, 5.0)
    rectangle = Rectangle.new(top_left, bottom_right)

    assert(rectangle.contains?(middle))
  end

  def test_rectangle_contains_edge
    top_left = Point.new(0.0, 10.0)
    bottom_right = Point.new(10.0, 0.0)
    point1 = Point.new(10.0, 5.0)
    point2 = Point.new(5.0, 10.0)
    rectangle = Rectangle.new(top_left, bottom_right)

    assert(rectangle.contains?(point1))
    assert(rectangle.contains?(point2))
  end

  def test_rectangle_not_contains
    top_left = Point.new(0.0, 10.0)
    right_left = Point.new(10.0, 0.0)
    outside = Point.new(100.0, 100.0)
    rectangle = Rectangle.new(top_left, right_left)

    refute(rectangle.contains?(outside))
  end
end
