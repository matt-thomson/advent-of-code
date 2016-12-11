defmodule Day07.Address do
  @moduledoc """
  Represents an IP address.
  """
  alias Day07.Abba
  alias Day07.Address

  @regex ~r{[\[\]]}

  defstruct [:supernet, :hypernet]

  def parse(input) do
    {supernet, hypernet} =
      input
      |> String.split(@regex)
      |> Enum.with_index
      |> Enum.partition(fn {_, idx} -> rem(idx, 2) == 0 end)

    %Address{
      supernet: supernet |> Keyword.keys,
      hypernet: hypernet |> Keyword.keys
    }
  end

  def tls?(%Address{supernet: supernet, hypernet: hypernet}) do
    cond do
      hypernet |> Enum.any?(&Abba.has_abba?/1) -> false
      supernet |> Enum.any?(&Abba.has_abba?/1) -> true
      true                                     -> false
    end
  end
end
