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

  def test_rectangle_does_not_contains
    top_left = Point.new(0.0, 10.0)
    bottom_right = Point.new(10.0, 0.0)
    rectangle = Rectangle.new(top_left, bottom_right)

    above = Point.new(top_left.x, top_left.y + 1)
    left = Point.new(top_left.x - 1, top_left.y - 1)
    below = Point.new(bottom_right.x, bottom_right.y - 1)
    right = Point.new(bottom_right.x + 1, bottom_right.y)

    refute(rectangle.contains?(above))
    refute(rectangle.contains?(left))
    refute(rectangle.contains?(below))
    refute(rectangle.contains?(right))
  end
end
