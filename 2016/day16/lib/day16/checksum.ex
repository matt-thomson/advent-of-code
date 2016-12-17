defmodule Day16.Checksum do
  @moduledoc """
  Computes the checksum for a string.
  """
  def calculate(input) do
    if input |> String.length |> rem(2) == 1 do
      input
    else
      input |> step |> calculate
    end
  end

  def step(input) do
    input
    |> to_char_list
    |> Enum.chunk(2)
    |> Enum.map(&handle_pair/1)
    |> Enum.join
  end

  defp handle_pair([x, x]), do: '1'
  defp handle_pair([_, _]), do: '0'
end
