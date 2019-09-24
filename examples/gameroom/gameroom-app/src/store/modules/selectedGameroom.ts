// Copyright 2019 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { Gameroom } from '@/store/models';

export interface SelectedGameroom {
  gameroom: Gameroom;
}

const selectedGameroom = {
  gameroom: ({} as Gameroom),
};

const getters = {
  getGameroom(state: SelectedGameroom): Gameroom {
    return state.gameroom;
  },
};

const actions = {
  async updateSelectedGameroom({ commit, rootGetters}: any, circuitID: string) {
      let gameroom: Gameroom = ({} as Gameroom) ;
      if (circuitID !== '') {
        const gamerooms: Gameroom[] = rootGetters['gamerooms/gameroomList'];
        gameroom =  gamerooms.find(
              (gr) => gr.circuit_id ===  circuitID) || {} as Gameroom;
      }
      commit('setSelectedGameroom', {gameroom});
  },
};

const mutations = {
  setSelectedGameroom(state: SelectedGameroom, {circuitID, gameroom}: any) {
    state.gameroom = gameroom;
  },
};

export default {
  namespaced: true,
  name: 'selectedGameroom',
  state: selectedGameroom,
  getters,
  actions,
  mutations,
};
