defmodule Day15.DiscTest do
  use ExUnit.Case

  alias Day15.Disc

  test "parses a disc" do
    input = "Disc #1 has 5 positions; at time=0, it is at position 4."
    disc = input |> Disc.parse

    assert disc.number == 1
    assert disc.positions == 5
    assert disc.start == 4
  end

  test "determines when a disc is in the correct position" do
    input = "Disc #2 has 2 positions; at time=0, it is at position 1."
    disc = input |> Disc.parse

    refute disc |> Disc.correct_position?(0)
    assert disc |> Disc.correct_position?(5)
  end
end
