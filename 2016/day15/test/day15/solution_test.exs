defmodule Day15.SolutionTest do
  use ExUnit.Case

  alias Day15.Disc
  alias Day15.Solution

  test "takes a step towards a solution" do
    input1 = "Disc #1 has 5 positions; at time=0, it is at position 4."
    disc1 = input1 |> Disc.parse

    input2 = "Disc #2 has 2 positions; at time=0, it is at position 1."
    disc2 = input2 |> Disc.parse

    solution = disc1 |> Solution.step(Solution.new)
    assert solution.push == 0
    assert solution.interval == 5

    solution = disc2 |> Solution.step(solution)
    assert solution.push == 5
    assert solution.interval == 10
  end
end
