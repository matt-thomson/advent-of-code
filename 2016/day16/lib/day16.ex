defmodule Day16 do
  @moduledoc """
  Entry point for day 16 solver.
  """
  alias Day16.Checksum
  alias Day16.Data

  def solve(input, length) do
    {length, ""} = length |> Integer.parse
    input |> Data.fill(length) |> Checksum.calculate
  end
end
