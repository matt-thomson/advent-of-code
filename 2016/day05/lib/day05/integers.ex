defmodule Day05.Integers do
  @moduledoc """
  Provides an infinite stream of integers.
  """

  def stream do
    0 |> Stream.iterate(&(&1 + 1))
  end
end
