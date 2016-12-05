defmodule Day05.PasswordTest do
  use ExUnit.Case

  alias Day05.Password

  test "selects the password character from a hash for part one" do
    assert Password.char_one(<<0, 0,  8, 100>>) == "8"
    assert Password.char_one(<<0, 0, 12, 100>>) == "c"
  end

  test "selects the password character from a hash for part two" do
    assert Password.char_two(<<0, 0,  8, 100>>) == "6"
    assert Password.char_two(<<0, 0, 12, 200>>) == "c"
  end

  test "selects the password position from a hash" do
    assert Password.position(<<0, 0,  6,   4>>) == 6
  end

  test "sets characters from a hash" do
    hash = <<0, 0, 1, 200>>

    assert [nil, nil, "a"] |> Password.set_char(hash) == [nil, "c", "a"]
    assert [nil, "b", "a"] |> Password.set_char(hash) == [nil, "b", "a"]
  end
end
