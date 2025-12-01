// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';

export class BattleResultImpl implements BattleResult {
  public readonly winner: BattleWinner;
  public readonly attackerPersonnel: ArmyPersonnelImpl;
  public readonly attackerSurvivingPersonnel: ArmyPersonnelImpl;
  public readonly defenderPersonnel: ArmyPersonnelImpl;
  public readonly defenderSurvivingPersonnel: ArmyPersonnelImpl;
  public readonly wallLevel: BuildingLevel;
  public readonly lossesRatio: number;

  private constructor(result: BattleResult) {
    this.winner = result.winner;
    this.attackerPersonnel = ArmyPersonnelImpl.create(result.attackerPersonnel);
    this.attackerSurvivingPersonnel = ArmyPersonnelImpl.create(result.attackerSurvivingPersonnel);
    this.defenderPersonnel = ArmyPersonnelImpl.create(result.defenderPersonnel);
    this.defenderSurvivingPersonnel = ArmyPersonnelImpl.create(result.defenderSurvivingPersonnel);
    this.wallLevel = result.wallLevel;
    this.lossesRatio = result.lossesRatio;
  }

  public static create(result: BattleResult) {
    if (result instanceof BattleResultImpl) {
      return result;
    }

    return new BattleResultImpl(result);
  }
}
