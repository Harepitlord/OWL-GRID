/**
 * Copyright 2018-2021 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

window.$CANOPY.registerConfigSapling('oauth-login', () => {
  console.log('Registering OAuth Login Sapling');

  if (window.location.pathname === '/oauth-login') {
    window.$CANOPY.registerApp(function (domNode) {
      console.log('Rendering OAuth-Login JS App');
      domNode.innerHTML = `<h1>OAuth Login<h1>`;
    });
  }
});
