defmodule Day19 do
  @moduledoc """
  Entry point for day 19 solver.
  """

  def part_one(input) do
    {input, ""} = input |> Integer.parse

    1..input |> Enum.to_list |> solve_part_one
  end

  defp solve_part_one([winner]), do: winner
  defp solve_part_one(elves) do
    num_elves = elves |> Enum.count
    if num_elves |> rem(2) == 1 do
      last_elf = elves |> List.last
      other_elves = elves |> Enum.drop(-1)
      [last_elf | other_elves |> step_part_one] |> solve_part_one
    else
      elves |> step_part_one |> solve_part_one
    end
  end

  defp step_part_one(elves) do
    elves |> Enum.chunk(2) |> Enum.map(fn ([elf, _]) -> elf end)
  end
end
