defmodule Day14.HashTest do
  use ExUnit.Case

  alias Day14.Hash

  test "finds the first triple" do
    hash = 18 |> Hash.new("abc")
    assert hash |> Hash.triple == ?8
  end

  test "finds fives" do
    hash = 816 |> Hash.new("abc")
    assert hash |> Hash.fives == [?E]
  end
end
