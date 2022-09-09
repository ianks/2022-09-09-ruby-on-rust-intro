require "test_helper"
require "demos/c"

class DemosCTest < Minitest::Test
  def test_reverses_ascii
    assert_equal("hello", reverse_it_c("olleh"))
  end

  def test_reverses_utf8
    skip "utf8 is not supported here :("

    assert_equal("hello 👋", reverse_it_c("👋 olleh"))
  end
end
