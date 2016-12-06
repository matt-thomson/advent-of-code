defmodule Day06 do
  @moduledoc """
  Entry point for day 6 solver.
  """
  alias Day06.Message

  def part_one(filename) do
    filename |> solve(&Message.most_common/1)
  end

  def part_two(filename) do
    filename |> solve(&Message.least_common/1)
  end

  defp solve(filename, select_char) do
    filename
    |> File.stream!
    |> Message.group_letters
    |> Enum.map(select_char)
    |> Enum.join
  end
end
