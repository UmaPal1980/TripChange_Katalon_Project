<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>U-SearchDomainPNR</name>
   <tag></tag>
   <elementGuidId>94e52208-e3a5-4f15-bc57-2eb8a6185513</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;CatalogOfferingsQueryAirChange\&quot;: {\n        \&quot;CatalogOfferingsAirChangeRequest\&quot;: {\n            \&quot;@type\&quot;: \&quot;CatalogOfferingsAirChangeRequestReservation\&quot;,\n            \&quot;returnBrandedFaresInd\&quot;: true,\n            \&quot;detailViewInd\&quot;: true,\n            \&quot;SearchCriteriaFlight\&quot;: [{\n                    \&quot;departureDate\&quot;: \&quot;${sExch_departureDate}\&quot;,\n                    \&quot;From\&quot;: {\n                        \&quot;cityOrAirport\&quot;: \&quot;Airport Only\&quot;,\n                        \&quot;value\&quot;: \&quot;${sExch_From}\&quot;\n                    },\n                    \&quot;To\&quot;: {\n                        \&quot;cityOrAirport\&quot;: \&quot;Airport Only\&quot;,\n                        \&quot;value\&quot;: \&quot;${sExch_To}\&quot;\n                    }\n                }\n            ],\n            \&quot;PricingModifiersAirChange\&quot;: {},\n            \&quot;BuildFromReservationWorkbench\&quot;: {\n                \&quot;ReservationIdentifier\&quot;: {\n                    \&quot;Identifier\&quot;: {\n                        \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n                        \&quot;value\&quot;: \&quot;${IdentifierResWorkBench}\&quot;\n                    }\n                },\n                \&quot;OfferIdentifier\&quot;: {\n                    \&quot;id\&quot;: \&quot;offer_1\&quot;\n                },\n                \&quot;ProductIdentifier\&quot;: [{\n                        \&quot;id\&quot;: \&quot;product_1\&quot;\n                    }\n                ]\n            }\n        }\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>traceId</name>
      <type>Main</type>
      <value>Search-10012022_1</value>
      <webElementGuid>02c180f9-295b-470e-885e-ff97d592597e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>41bf0857-2fb0-41ff-b53e-67d13dd5b23b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>682789f1-6761-48bf-9b21-a61ae57ccf57</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>155da7ca-06bb-40df-8191-ab4b46f2091a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>mergeDiagFlag</name>
      <type>Main</type>
      <value>false</value>
      <webElementGuid>62494a3f-94ca-4d36-bbdc-a628204ef7d6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAuth_ClientID</name>
      <type>Main</type>
      <value>qa-sXZ3EygiKuovA7RgGjDbZ1PIRimxetbaAGczeGnW</value>
      <webElementGuid>7d84148a-beb8-4174-bc24-a83e11261e93</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>Locale=en_US,lastName=testuser,accessGroups=cn=512928,ou=Organization,dc=travelport,dc=com^cn=504860,ou=Organization,dc=travelport,dc=com^cn=AB6922CC-BE07-4291-B38F-AB25ED96F07E,ou=AccessGroups,dc=travelport,dc=com^cn=7C7ED10A-EEBC-4468-B499-879DE63F1B7D,ou=AccessGroups,dc=travelport,dc=com^cn=D6BBCAA7-76AE-493F-87DD-7D32F12EE31B,ou=AccessGroups,dc=travelport,dc=com,accessList=512928^504860^AB6922CC-BE07-4291-B38F-AB25ED96F07E^7C7ED10A-EEBC-4468-B499-879DE63F1B7D^D6BBCAA7-76AE-493F-87DD-7D32F12EE31B,Roles=Developer^Travel Agent,SSOSessionProperties={\\\&quot;SmAuthResults\\\&quot;:\\\&quot;AQFzABVUUzFHdGVzdHVzZXJAbWFpbC5jb21zABxOdEk3K1FKOWs1RVowbkJaYzBaNXZNUnNMYms9cwHsYXUyMUhxeEdaTW1LTXlOYUYwN3FtRTlscG5IRVdnM0hqTlYvQ0IzMzdqeHhjekJMK3FCSGpjdTNWVllaTEtzNW1wLzVSUGYxVmo3S1pVbGI2ODlDYS9kTTE5YW8rRWlHaHhUQTNKcnVlZloyR1p0NjRZQXU5Y0RNSlkzVTB4L1pLeG5mZmpQY3Zubm41Y0hvb2hDaU5rRVBvRmZQTWMwSDVjdDN5ZnlVUVZoZVVGUU43NGRaeEdxSjhtN1ZhdDlzNEZPeVZ1Wk10dVBQZ1JPWncrQ3JNc2c3TTVGNDlsWDljTyszdWRJTWhBWExkekV4MVpGeWpudHBXSjY0RERZR2JvQU0xbGlIbHI1eUlVdE1XUjJoMDJZK0Y5UXhra1ZqbStRck0wL295bUlSZzFiSG9BZGNRVm1XNE5rVkZMc1pzMGw3MDJ1V0JIRHVTOURkUG5CU0c3bSszWVlpV2gycTdIdjMzVmFMbWUvNVVPLzJiY3g2T0M1TFE3eVdWNUZOcWJEa1Y3RlFua0x6MmxiVjdIR20zMnltRzlwSFNESkRlTnYwZ2lMcm4xR0tmcmY4YXg4cko2Q0p4S1hhN3F1aFJ0cldva1h2aWN2NkpmWWduVUN3dE1VRGlXT2ZiMEs1b0JNaEdUc0o0Q3c9CAAAAXVXTpCAAjhAAnCABF+S1tAEX5LW0ARfktbS\\\&quot;},firstName=TRIPSERVICES,UID=TP98348858,Language=English,Static_Gtids=,email=TS1Gtestuser@mail.com, forwarded=for=10.107.131.31;host=ts-airbook-res-session-do-6-qab-tripservices-qa.ocp-a.zu2.nonprod.travelport.io:443;proto=https;proto-version=, xauth_travelport_accessgroup=7C7ED10A-EEBC-4468-B499-879DE63F1B7D, x-forwarded-port=443, e2etrackingid=IntegratedDemo:qand1001}</value>
      <webElementGuid>a1d15194-f5f1-4bb3-a5a9-4c93498f7069</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJkaXIiLCJlbmMiOiJBMTI4Q0JDLUhTMjU2IiwianRpIjoidjkwbG1tYzBubzEzIn0..2GvS_K36wnbKsI7FKWRLCw.7IKosFl5l4_5JD5SF_FjMAxXCzofSexicLHGkvAyKVXgtETWp-mVmWxqguF-Y8QdXWiPUU9dBnDgyuWiPrYbx3Hqd2gIdtF0Qr2twR3mAe4Q_inTp0STTz9pSh6OiGqO2s8a5IRRiAuU86kh0PLzB3dOTRKNsTRbi_9a8We9uGPfMmWdC2M8A-dbqLE1wDxhu5_pIK_UKxZ4ikKvOed2tf88vmb-M3YezJNaZEdlTNGv4KnfKcMfYO8pIfWFWjg_6mmP-uuNiEMOd3BJbQ8LpueiI8dFexAWTwWiajLt8NNw7o52c0Pt03D659YPtYJTmU2wVXYHEUVY0UYbdgu33ZVBQBm9cyPnxJ5Ez7gEHpayAA0KWQZVCeX0B1DvdQ1u2GbR_Kb4u0Q0pN2vNByJeVguVfkr-jsHMpWpIcNUhhVnq_Hqx-mdtgrGrgalKkabVQoA5wy1N5rvSkVKtsJ1I6BQiMojLMu8nUkW9KjImLL729J-vwtLfvDN5MLvoWeVCaR3Lm8Js6lKoJay4PPXIbDDWWg6C7wlLZ0j7JWSgzIB_dv5I5CQG09U7kCPuvyrT3qfMjZeIKLfCsXjkdBNc96wj6J2R4ftwykmHhDli4dOSu963Zkvwlx2yBSnIQzLh_D-rQ-ikZHFY1y2P2TArXPSqmHP5HuMnXJzQHuCOC1GnPVDChl0AF5a1eGaOmKyr9o5p0QMoNdIbbipz4SymxNwXXMUfL9aA2SktAc77FSvSLZtHVjYofieK2I2BdTZLmFJJ8ChSqNBYkB4sFjc5fzoZ4wjFJTBB8u9HuiuHLisAq3Qu1PImjIOLzYBhRkmcpCKXSG02rxgRKpJJ0X2qHNu3lMPk4KJSKO8ZjNSlXiXwYLDljy1OBrMKkNkaT9XQqIkS7NK45t3mro2B07yjU47IHkrc21E1e3OHykIEPXQgkQNGrywSv0N0AtWPe43R3J8IWTleQAjilPtyxknx_1hMGRsJtT4EZu6ke7Ga7IfpbAl7VGsZ92iwn9MDMah3Zv0wJhc0Kpgy0CIbNQTmmF-Pn6J5yx-ggktXu8BOutI4Wl-aR1wIzoYkWi-UpIJO4cJbKjSoBTXcxVIwVTj7yO3-Z4WYC9XpMJCSneGS1XkjSXQyb8PfoYo0OuB_tzcDuHeyT2NiWtpf5axjqh3aeXriuDzSc_rNMGRY_80_qebIfzF4CZ8JGSNdtni5Z5l-rCdjpr3jwhDqT-jJO-EVfHexidYCX7RyzztqQu8KfhEKvI0H9ag_5SHoPx0DhvRrLwfKojxX1iEBGn2vntgDYE92TIjGnxyav1Hfsekzi4WwRbhMiS01imUiCuxOiiDJapd9EjmF7F3elwa2jBaDzfYdRn0vTcs0pBpQjt51HLAxARkBMR9O88gGMXPE-hqdo7pqAhr9AbfpKVYICuRtSbfb4ELNOul2xcdn5pwSK4aUBfTb9QlL5D6leBb-HWlcH_3RHF0Q9TgtzmQ_UVG0ZdSHfo5NNEbmMY-X8B1lVqsIKrfvC9l7rlqsIHDToK57xBn2KgcaLcG1gz7CKRL98hdR3mtNKa1O7pRTz_n8LabHPSi3uyU3UyJyAAkftXFszttvbtLpp-seUYMRb1OQn6DV3Ufo8ke5lzKQSvp5q4DgNO3pWfrGAmDm0BFFlI-XYceRg8PWaxm-XcRtwU9q-0GFLZEEYY-ur_rLTBb_aJzPTcekI9Vu4lkeCghMok-dIzq5DDUfuM0VPDWekX4CKJQogBCBfNicUxWm0Zce1APHjIe0do6YTtzD7hZUscQ-4NbkfyWlzhur74Mm-FKZTpluE4Ypi7UziH71fwB4zETvkVV06MIHc3iBONCrG45juYs4R6iSCR77mLg0ofJkYjvhD9EtGePN_lEX2v-Ssf5S5dTE9ltSgbPRhxiu1wZVLVl4yRP-7NdqFw0dCzB9g4AA0D2L26VsT5oTSEWvmtNgXALB6Ec1wlZR_eCYXS325HmpGKng0XC9wRtZUM8dZdAz274KF1Dt_uiKj2Q-MdkcMUjpN19FCmduKL4G2nnlHsHqQm4ICxkEEPl7YmP0MpWt1pxr2xucRkGb6RpMQk3qDPm_azKaMAXNQJTVPg4a4vJDsmXnLq11GIqMCJW4UjizT-HyDEWrHg7M55hiQQ5Bbf-UvszHejL7rKfLW3vp4sNjzTOlyIemq7RXmqS7m1xps-7LTfnQW6PvOCL65EpU71Y5NEYEsxTcCAuqbHx0S8Ae_R8toPERF8S8nMHjnC27DWbKSOhohGiBHOesrJlANJmPs8677pGRorTuVIK45IQffTpWs8o_ZqPHdsVxH1NXwgnk6oI_BTSjaeZc_bv-yIcVzbWpI6EX-iQFJoMpg2CoujBHE9qLSjc0RhYg80DZA-YLvf8Uet1zxxuwRLraAtBaQrEd_gQZypbJSAmAC5cCz3gPZnzyiW1G9d_BW_y82G57F842gqKn-iaLwjufiPwGlLkEQVY8sWYzCAn_b5tAMHZdFgpSdK6AjrLPz3Uqq_ldGU5hN-NVor8JUmkznaM7j9uCu-jj3ocktCxvoQK0yVJfvv7r_K3a-sfnuatRUSz5NMC55uobxjj6E20NZPMzOEVZKU4rQkHPESoGBfz-XFDb_ftglQ-uEWffE0xwSOvzdGokEVPKCJZ9Zm2_XNY7QlD33XzSqE6LdX7mcQlmPfK5G3eOjVZAePbyIwwpd-24pQKrplQYVZ--QiiGkXoB6BGvbXxMy6AIh83Kula3Kh7l7rJPkQsmoSos0LEIpKIYyi7WzzIU_nTq4KQzK3-6Re4Ogh6ORMdbliggIj7DWS17Zp9MJ_d-pOOCdC7h3MvzDLP5bDEhLlv60D-bUIvf_AmDxLaP5yYAydg0zrBDr1nwz5pz9-7tXEe8cUulQ9exqAphyG_a8rzfBryLiQixkwImoYmI4U7QUEcHSiOxIhW4etxFfTi7Dg6l2STQZmaDfMCnlSjE7ulU9CEmjK0_sISlTNRFeOumDd8HUZ2mbQp0t16y5NUk7GIpLgXah-Q5AGGNVhgM4kCYVXve8V_GeBVc-iFXGc_DWiSFdAu_l_zS57Tq22RLnbcyaxWS2OYIhp2EOFPZuvw3L-nCDDR9OYKIAowa2w2G6v4oUebzfIwTPWLAyGr1dghY0L11cHWVvIikR7yGvU9WbxSZ-QJ2Gd6UVF9nd6f5FejalnsCUBsrSK6-H02rZuW4jViit-8WCs3iJR6A7G4Jttk95lqMe3v7MHW7rj4MgD3ISn6mT4jkspTXdQQ7-eV2Ls8T9rBL3qw6KNh2zaYT1uYlU0DfZdPx7-f2YCXV-6z8-qc-zRUr0gRY0bBdolB7mUQzDbwaXy4ph8M_cakrNNYQBWWROnaN2bbYheFBVqcQKLAi8fup3vkjxaLM_vqsfPSKqWCy38kdBb3YhX5pK8-ZfZM0yDnytxEZLD6MuIg4yegSkhxdQmppiJqcl0uJWXW4F6jnRRiyapTQZokjEyu8NzUTKjH_W2hdCJ3MpNNM6G3fZLg8uMkdFpel_LHy-m3HqG3lsmeyTrjrZHk1T-Wcxm_SWwu8hTShUo6c_cG9TXcZmpj-wuoClLUaoPVdED16Ty3zmOdmuTpSFbp8drTo-nRiydD4bO9EnwEGUWPecUaPcA8QV4z-f2sJ8NXJxxBxLq9BeiHiZUhNfDpry4WpMa4zGjwqR3mJ2m2X4wN5qVYp-KFI5l5suFBekt8I31yv5TeLTDrhVVhrU1TKfhYMor0r3HxLs7S9yQaOYowXGKM2HXKdqvNDWVMmw4Y54Eapn8fpjzzfsnjGOIdnzUX8ZzkURZoCgguVPzI74k9iCVYXeDQ-ttmw-d4TvAPdbv-9or6zRnje_9RhrPtPbzzBLcXLQAbD5A2pvCO8WBM2nSL6OycuxmPq8Xq1Nauv1loeUZuWc6114gslFfdmdMrGRkKWTU_oPhHGJS14cHhNTmhTLiJe-HUpwPZYQWNF3fgjfDT-tF_bjKRibuM2szlXRmlyiUL2e5DAwJt7WyZjEn-O1RcH6_sWRzVy1z5ObFkqQuGwyKiz_LTimGoBDf44-xti9fKQ0Q40BhyCZJZr_SovPMWCy-yxWrnQQ7c95DSyPTjY5e9yrv_rUcp2mB6Vvf2Cq0ucLVF3lKB0y04_d3x4Yhlt3Waw5eRYN8hYhZopO_0y1xVtqjbDnH-NUsK7sSgUb8VB-AgwG1KRzRq_ao-fLAj4-nn9goTlBPCBu0rvpVjVNd7vqkBqUWAQRWmFCP4Llahl4_OgsmBAj_nxdP595Jc0iTYr4jdr_qgsJwnNoTVdHQEhFh26zGdH40Bst7rtM751FMrquiFpwQD0gCBFRtEzIvy0IpeLnKCDJ9nbtD1BLVjYgzby-GqIxxyYFO8ka_FKWE5TXy5OD5diDtX7oI8mKnwXsU7iPC8e9pvkd8RepDVUAbHfSCBy2zu-s-znhZvZsaUoNU-iIJIA-ST_Co04_bVudUH1aRdYV4tPkqUNnUXINkdHs7uK0gD_hsZFPDGAh25atyQRtDJJ_ARjiOzXSMXJXm3xWz26goYyvsHzq-OxQxmApCCfGckMW4kowaHGK-y4wythWgos_n_YDY6l4z45OOZPCYUJesfbcQ9d1-AjrmdVSZ3zZ6bJNSXkl2kc1tyHAzBi087exsVBnIkdetuXP581w2l0XQWo_Zs-Npr0MkgN3iHQwfq57sSEcS-mM2JMEolIJbODlakyXEJATQT5vvbu7KctDTMS5E5E-yJrJ84s9mJkBXZyg7krhfmc2EUCV1t4sqwLnvP_2Qq6Ln_7qNax2jfoME2yyLako5LFjRc1IoSfe1aoYaxSbB-CIjbFyeWkBnMcxNj6iGeHpwyGATacA2BjXl-HeiDidV_Vx-t8uWsIBvQy8VeNvjv7rkz7YF21D60IEkYiGE1o2MHqNY5g4SNpfnxrLkBdkmOSYzCpU4m9J_MH2RvIjcqiSpBFK2K6phvAu__IE4h6ux8o0DlG5CPD2cVUu-5G3rLGJp0QQwMk9Y9oqRNWpVe6IaLBO9U7iGsQhsHvBmAmWrs9Xdv3eIAxSeblbJ23ZAUttmuElvzh9nqzrjtGdybhtN5qLltsnTXy_VPlksp9hl8WJp2LCij8QnfvHXRhjyKMHmjd3XrBoOZZZTvNb3NDmSJmWgPBxCLXEwyyHsJf8cTBhF3-4SFrj8bt0TxR2ZzgqZe7MEW6Y6kLoEugzr5zqS5GpoZ5ehERX-O9gJukydPgVjJSPY-_Vm_-AkGUyQ_DsX5oTfCIGd-PghuFaB1R-FU6Mzt9MIxRAFrrAoBKP6V6fci_oJysB6s2XcspvCE1w569WFa1SjioQAgzxcjwhW24zFKWqkvQRv4vsXtB1dsyMJSudSbjg.jH7NkWgI6yuqAmUbBTVJWQ</value>
      <webElementGuid>7713e146-4a76-4d74-a07b-ab08c0cfadb0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>02fb595c-cb9d-447b-9aaa-e775c55271d9</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sSearchDomainPNRURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.IdentifierResWorkBench</defaultValue>
      <description></description>
      <id>06a0670c-7a0b-421e-92c8-64c111b2c34a</id>
      <masked>false</masked>
      <name>IdentifierResWorkBench</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>0bce009d-495d-4088-b510-3f012845e7cc</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sExch_From</defaultValue>
      <description></description>
      <id>c8cda32d-782b-442f-9f05-579012ff0a03</id>
      <masked>false</masked>
      <name>sExch_From</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sExch_To</defaultValue>
      <description></description>
      <id>992769b6-2abb-4f8a-9029-85d990732fd0</id>
      <masked>false</masked>
      <name>sExch_To</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sExch_departureDate</defaultValue>
      <description></description>
      <id>7aae04a3-9642-4396-a224-ccdd32e07ced</id>
      <masked>false</masked>
      <name>sExch_departureDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sExch_RetnDate</defaultValue>
      <description></description>
      <id>26ddab94-8c43-4ab1-8ca5-f8063d18ebf0</id>
      <masked>false</masked>
      <name>sExch_RetnDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sExch_Flight</defaultValue>
      <description></description>
      <id>e15fc425-7921-4ffa-9394-406ccba3cb51</id>
      <masked>false</masked>
      <name>sExch_Flight</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sSearchDomainPNRURL</defaultValue>
      <description></description>
      <id>cc6db06c-d487-4f72-aa71-f6b531a67729</id>
      <masked>false</masked>
      <name>sSearchDomainPNRURL</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
