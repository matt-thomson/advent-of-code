# frozen_string_literal: true

RSpec.describe AdventOfCode2020::Day05::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day05.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(820) }
  end
end
