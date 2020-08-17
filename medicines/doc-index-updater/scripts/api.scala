/*
 * Copyright 2011-2019 GatlingCorp (https://gatling.io)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package computerdatabase.advanced

import io.gatling.core.Predef._
import io.gatling.http.Predef._
import scala.concurrent.duration._
import java.util.concurrent.ThreadLocalRandom

class ProductsApi extends Simulation {

  object Search {

    val search = exec(
        http("Search")
          .post("")
          .body(StringBody("{\"query\":\"{ substance(name: \\\"CAFFEINE\\\") { products { name, documentCount } } }\",\"variables\":null}"))
          .check(status.is(200)))
        .pause(5)
  
  }

  val httpProtocol = http
    //.baseUrl("https://medicines-api.non-prod.mhra.gov.uk") // non prod
    .baseUrl("https://medicines-api-dev.test.mhra.gov.uk") //dev
    //.baseUrl("http://localhost:8000") // local
    .acceptHeader("application/json")
    .contentTypeHeader("application/json")
    .doNotTrackHeader("1")
    .acceptLanguageHeader("en-US,en;q=0.5")
    .acceptEncodingHeader("gzip, deflate")
    .userAgentHeader("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:16.0) Gecko/20100101 Firefox/16.0")

  val users = scenario("Users").exec(Search.search)

  setUp(
    users.inject(rampUsers(1000) during (10 seconds)),
  ).protocols(httpProtocol)
}
