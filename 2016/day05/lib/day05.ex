defmodule Day05 do
  @moduledoc """
  Entry point for day 5 solver.
  """
  alias Day05.Hash
  alias Day05.Integers

  def part_one(input) do
    Integers.stream
    |> Stream.map(&Hash.hash(input, &1))
    |> Stream.filter(&Hash.interesting?/1)
    |> Stream.map(&Hash.password_char/1)
    |> Stream.take(8)
    |> Enum.join
  end
end
