# frozen_string_literal: true

require 'advent_of_code_2020/day04/validators/eye_colour'

RSpec.describe AdventOfCode2020::Day04::Validators::EyeColour do
  subject(:validator) { described_class.new }

  describe '#valid?' do
    subject(:valid?) { validator.valid?(input) }

    context 'with a valid input' do
      let(:input) { 'brn' }

      it { is_expected.to eq(true) }
    end

    context 'with an invalid input' do
      let(:input) { 'wat' }

      it { is_expected.to eq(false) }
    end
  end
end
