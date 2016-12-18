defmodule Day18 do
  @moduledoc """
  Entry point for day 18 solver.
  """
  alias Day18.Row

  def solve(filename, rows) do
    {rows, ""} = rows |> Integer.parse

    filename
    |> File.read!
    |> String.strip
    |> Row.parse
    |> Stream.iterate(&Row.step/1)
    |> Stream.map(&Row.count_safe/1)
    |> Enum.take(rows)
    |> Enum.sum
  end
end
