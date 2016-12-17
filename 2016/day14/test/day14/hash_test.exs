defmodule Day14.HashTest do
  use ExUnit.Case

  alias Day14.Hash

  test "finds the first triple" do
    hash = 18 |> Hash.new("abc", 1)
    assert hash |> Hash.triple == ?8
  end

  test "finds fives" do
    hash = 816 |> Hash.new("abc", 1)
    assert hash |> Hash.fives == [?e]
  end

  test "hashes through multiple iterations" do
    hash = 5 |> Hash.new("abc", 2017)
    assert hash |> Hash.triple == ?2
  end
end
