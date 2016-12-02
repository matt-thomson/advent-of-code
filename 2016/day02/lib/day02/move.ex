defmodule Day02.Move do
  alias Day02.Step

  def move(directions, position) do
    directions |> Enum.reduce(position, &Step.step/2)
  end
end
