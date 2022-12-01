# frozen_string_literal: true

require 'advent_of_code_2020/day04/validators/height'

RSpec.describe AdventOfCode2020::Day04::Validators::Height do
  subject(:validator) { described_class.new }

  describe '#valid?' do
    subject(:valid?) { validator.valid?(input) }

    context 'with a valid input in inches' do
      let(:input) { '60in' }

      it { is_expected.to be(true) }
    end

    context 'with a valid input in centimetres' do
      let(:input) { '190cm' }

      it { is_expected.to be(true) }
    end

    context 'with a too large input in inches' do
      let(:input) { '190in' }

      it { is_expected.to be(false) }
    end

    context 'with no unit' do
      let(:input) { '190' }

      it { is_expected.to be(false) }
    end
  end
end
