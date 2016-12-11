defmodule Day07.Address do
  @moduledoc """
  Represents an IP address.
  """
  alias Day07.Aba
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

  def ssl?(%Address{supernet: supernet, hypernet: hypernet}) do
    abas =
      supernet
      |> Enum.flat_map(&Aba.abas/1)
      |> MapSet.new

    babs =
      hypernet
      |> Enum.flat_map(&Aba.abas/1)
      |> Enum.map(&Enum.reverse/1)
      |> MapSet.new

    abas |> MapSet.intersection(babs) |> Enum.any?
  end
end
