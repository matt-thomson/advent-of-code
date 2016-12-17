defmodule Day15.Solution do
  @moduledoc """
  Represents a step in finding the start time.
  """
  alias Day15.Disc
  alias Day15.Solution

  defstruct [:push, :interval]

  def new, do: %Solution{push: 0, interval: 1}

  def step(disc, solution) do
    if disc |> Disc.correct_position?(solution.push) do
      %{solution | interval: solution.interval * disc.positions}
    else
      solution = %{solution | push: solution.push + solution.interval}
      disc |> step(solution)
    end
  end
end
