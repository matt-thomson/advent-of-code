# frozen_string_literal: true

require 'advent_of_code2017/day04'

RSpec.describe AdventOfCode2017::Day04 do
  describe '.solve_part_one' do
    subject { described_class.solve_part_one(input) }

    let(:input) do
      <<~INPUT
        aa bb cc dd ee
        aa bb cc dd aa
        aa bb cc dd aaa
      INPUT
    end

    it { is_expected.to eq(2) }
  end

  describe '.solve_part_two' do
    subject { described_class.solve_part_two(input) }

    let(:input) do
      <<~INPUT
        abcde fghij
        abcde xyz ecdab
        a ab abc abd abf abj
        iiii oiii ooii oooi oooo
        oiii ioii iioi iiio
      INPUT
    end

    it { is_expected.to eq(3) }
  end
end
