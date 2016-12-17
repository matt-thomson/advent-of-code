defmodule Day14 do
  @moduledoc """
  Entry point for day 14 solver.
  """
  alias Day14.Hash
  alias Day14.Integers

  def part_one(input) do
    input
    |> hashes
    |> Stream.chunk(1001, 1)
    |> Stream.filter(&key?/1)
    |> Enum.at(63)
    |> Enum.at(0)
    |> Hash.index
  end

  defp hashes(input) do
    Integers.stream |> Stream.map(&Hash.new(&1, input))
  end

  defp key?([hash | next_hashes]) do
    triple = hash |> Hash.triple
    triple && next_hashes |> Enum.any?(&Hash.has_five?(&1, triple))
  end
end
