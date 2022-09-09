require "test_helper"
require "demos/rust_rbsys"

class DemosRustRbSysTest < Minitest::Test
  def test_reverses_ascii
    assert_equal("hello", reverse_it_rust_rbsys("olleh"))
  end

  def test_reverses_utf8
    assert_equal("hello 👋", reverse_it_rust_rbsys("👋 olleh"))
  end
end
