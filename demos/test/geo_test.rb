require "minitest/autorun"
require "demos/rust_magnus_geo"

class GeoTest < Minitest::Test
  def test_point_distance
    a = Point.new(0.0, 0.0)
    b = Point.new(0.0, 1.0)

    assert_equal(1.0, a.distance(b))
  end

  def test_rectangle_contains_true
    top_left = Point.new(0.0, 10.0)
    bottom_right = Point.new(10.0, 0.0)
    middle = Point.new(5.0, 5.0)
    rectangle = Rectangle.new(top_left, bottom_right)

    assert(rectangle.contains?(middle))
  end

  def test_rectangle_contains_false
    top_left = Point.new(0.0, 10.0)
    bottom_right = Point.new(10.0, 0.0)
    middle = Point.new(5.0, 15.0)
    rectangle = Rectangle.new(top_left, bottom_right)

    refute(rectangle.contains?(middle))
  end
end
