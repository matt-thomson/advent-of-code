defmodule Day16.Data do
  @moduledoc """
  Generates data to fill a disc.
  """
  def step(input) do
    input <> "0" <> (input |> invert)
  end

  def fill(input, length) do
    if input |> String.length > length do
      input |> String.slice(0, length)
    else
      input |> step |> fill(length)
    end
  end

  defp invert(input) do
    input
    |> String.replace("0", "x")
    |> String.replace("1", "0")
    |> String.replace("x", "1")
    |> String.reverse
  end
end
