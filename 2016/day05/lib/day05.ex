defmodule Day05 do
  @moduledoc """
  Entry point for day 5 solver.
  """
  alias Day05.Hash
  alias Day05.Integers
  alias Day05.Password

  def part_one(input) do
    input
    |> interesting_hashes
    |> Stream.map(&Password.char_one/1)
    |> Stream.take(8)
    |> Enum.join
  end

  def part_two(input) do
    input
    |> interesting_hashes
    |> Enum.reduce_while(nil |> List.duplicate(8), &reduce_hashes/2)
    |> Enum.join
  end

  defp interesting_hashes(input) do
    Integers.stream
    |> Stream.map(&Hash.hash(input, &1))
    |> Stream.filter(&Hash.interesting?/1)
  end

  defp reduce_hashes(hash, characters) do
    characters = characters |> Password.set_char(hash)

    if characters |> Enum.any?(&is_nil/1) do
      {:cont, characters}
    else
      {:halt, characters}
    end
  end
end
