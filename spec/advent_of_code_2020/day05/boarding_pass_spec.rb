# frozen_string_literal: true

require 'advent_of_code_2020/day05/boarding_pass'

RSpec.describe AdventOfCode2020::Day05::BoardingPass do
  subject(:boarding_pass) { described_class.new(input) }

  describe '#seat_id' do
    subject(:seat_id) { boarding_pass.seat_id }

    context 'with first example' do
      let(:input) { 'FBFBBFFRLR' }

      it { is_expected.to eq(357) }
    end

    context 'with second example' do
      let(:input) { 'BFFFBBFRRR' }

      it { is_expected.to eq(567) }
    end

    context 'with third example' do
      let(:input) { 'FFFBBBFRRR' }

      it { is_expected.to eq(119) }
    end

    context 'with fourth example' do
      let(:input) { 'BBFFBBFRLL' }

      it { is_expected.to eq(820) }
    end
  end
end
