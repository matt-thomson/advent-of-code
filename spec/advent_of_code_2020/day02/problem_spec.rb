# frozen_string_literal: true

RSpec.describe AdventOfCode2020::Day02::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day02.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(2) }
  end

  describe '#part_two' do
    subject(:part_two) { problem.part_two }

    it { is_expected.to eq(1) }
  end
end
