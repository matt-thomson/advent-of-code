# frozen_string_literal: true

module AdventOfCode2017
  module Day03
    def self.solve_part_one(number)
      circuit = (Math.sqrt(number - 1).floor + 1) / 2
      distance_to_nearest_midpoint =
        midpoints(circuit).map { |midpoint| (midpoint - number).abs }.min

      distance_to_nearest_midpoint + circuit
    end

    def self.midpoints(circuit)
      base = 4 * circuit * circuit + 1
      [
        base - 3 * circuit,
        base - 1 * circuit,
        base + 1 * circuit,
        base + 3 * circuit
      ]
    end
    private_class_method :midpoints
  end
end
