defmodule Day13.Route do
  @moduledoc """
  Computes the shortest route to the destination.
  """
  alias Day13.Cubicle
  alias Day13.Route

  defstruct [:input, :confirmed, :unconfirmed]

  def new(input) do
    %Route{input: input, confirmed: %{}, unconfirmed: %{{1, 1} => 0}}
  end

  def find(route, destination) do
    (route |> distance(destination)) || (route |> step |> find(destination))
  end

  def cubicles_within(route, steps) do
    if route |> unconfirmed_within(steps) do
      cubicles_within(route |> step, steps)
    else
      route |> number_confirmed
    end
  end

  def step(route = %Route{input: input,
                          confirmed: confirmed,
                          unconfirmed: unconfirmed}) do
    {next, distance} =
      unconfirmed
      |> Enum.min_by(fn ({_, distance}) -> distance end)

    confirmed = confirmed |> Map.put(next, distance)
    unconfirmed = unconfirmed |> Map.delete(next)

    neighbours =
      next
      |> Cubicle.neighbours
      |> Enum.reject(&Cubicle.wall?(&1, input))

    unconfirmed =
      neighbours
      |> Enum.reduce(unconfirmed,
                     &handle_neighbour(&1, &2, distance, confirmed))

    %{route | confirmed: confirmed, unconfirmed: unconfirmed}
  end

  def distance(%Route{confirmed: confirmed}, target), do: confirmed[target]

  defp handle_neighbour(neighbour, unconfirmed, distance, confirmed) do
    unconfirmed_distance = unconfirmed[neighbour]

    cond do
      confirmed |> Map.has_key?(neighbour) ->
        unconfirmed
      !(unconfirmed |> Map.has_key?(neighbour)) ->
        unconfirmed |> Map.put(neighbour, distance + 1)
      unconfirmed_distance <= distance ->
        unconfirmed
      true ->
        unconfirmed |> Map.put(neighbour, distance + 1)
    end
  end

  defp unconfirmed_within(%Route{unconfirmed: unconfirmed}, steps) do
    unconfirmed |> Map.values |> Enum.any?(&(&1 <= steps))
  end

  defp number_confirmed(%Route{confirmed: confirmed}) do
    confirmed |> Enum.count
  end
end
