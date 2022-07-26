<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>1_Search_CT</name>
   <tag></tag>
   <elementGuidId>df3b459a-5418-47c3-bae1-8715c30a5a52</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;CatalogOfferingsQueryRequest\&quot;: {\n    \&quot;CatalogOfferingsRequest\&quot;: [\n      {\n        \&quot;@type\&quot;: \&quot;CatalogOfferingsRequestAir\&quot;,\n        \&quot;maxNumberOfOffersToReturn\&quot;: 200,\n        \&quot;offersPerPage\&quot;: 200,\n        \&quot;contentSourceList\&quot;: [\n          \&quot;GDS\&quot;\n        ],\n        \&quot;returnBrandedFaresInd\&quot;: true,\n        \&quot;detailViewInd\&quot;: true,\n        \&quot;upsellInd\&quot;: true,\n        \&quot;PassengerCriteria\&quot;: [\n          {\n            \&quot;@type\&quot;: \&quot;PassengerCriteria\&quot;,\n            \&quot;number\&quot;: \&quot;${sPassengerCount}\&quot;,\n            \&quot;passengerTypeCode\&quot;: \&quot;${sPTC}\&quot;\n          }\n        ],\n        \&quot;SearchCriteriaFlight\&quot;: [\n          {\n            \&quot;departureDate\&quot;: \&quot;${sDepDate}\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;${sFrom}\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;${sTo}\&quot;\n            }\n          },\n          {\n            \&quot;departureDate\&quot;: \&quot;${sCircleDate}\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;${sTo}\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;${sCircleTripApt}\&quot;\n            }\n          },\n\t  {\n            \&quot;departureDate\&quot;: \&quot;${sRetDate}\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;${sCircleTripApt}\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;${sFrom}\&quot;\n            }\n          }\n        ],\n \t\&quot;SearchModifiersAir\&quot;: {\n          \&quot;excludeGround\&quot;: \&quot;Train\&quot;,\n          \&quot;FlightType\&quot;: {\n            \&quot;connectionType\&quot;: \&quot;DoubleConnection\&quot;,\n            \&quot;excludeInterlineConnectionsInd\&quot;: true\n          },\n          \&quot;CarrierPreference\&quot;: [\n            {\n              \&quot;preferenceType\&quot;: \&quot;Preferred\&quot;,\n              \&quot;carriers\&quot;: [\n                \&quot;${sCarrier}\&quot;\n              ]\n            }\n          ],\n          \&quot;CabinPreference\&quot;: [\n            {\n              \&quot;preferenceType\&quot;: \&quot;Preferred\&quot;,\n              \&quot;cabins\&quot;: [\n                \&quot;Economy\&quot;\n              ]\n            }\n          ],\n          \&quot;ConnectionPreferences\&quot;: [\n            {\n              \&quot;@type\&quot;: \&quot;ConnectionPreferencesAir\&quot;,\n              \&quot;FlightType\&quot;: {\n                \&quot;connectionType\&quot;: \&quot;DoubleConnection\&quot;,\n                \&quot;excludeInterlineConnectionsInd\&quot;: true\n              }\n            }\n          ]\n        },\n        \&quot;PricingModifiersAir\&quot;: {\n          \&quot;@type\&quot;: \&quot;PricingModifiersAir\&quot;,\n          \&quot;currencyCode\&quot;: \&quot;${sCurrencyCode}\&quot;,\n          \&quot;FareSelection\&quot;: {\n            \&quot;@type\&quot;: \&quot;FareSelectionDetail\&quot;,\n            \&quot;prohibitMaxStayFaresInd\&quot;: true,\n            \&quot;fareType\&quot;: \&quot;PublicFaresOnly\&quot;\n          }\n        }\n      }\n    ]\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>943b6672-53de-4348-857e-403467a81f65</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bd149f0d-e8ff-421e-bd30-6c889bf6baf7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>b8ba27c0-425d-4606-afcd-1ee30bc6ddc5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>e3de4a43-5b36-4d53-9fed-ee90977e758c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>43ccf549-a791-4546-a1a3-379c241a93ed</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>2d392385-241a-4084-988d-2fd62214734a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>6d7e5b50-b0bd-4f39-921d-7209ea6f2113</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>8fae72e1-f643-45f5-ab2c-d217a57ad9ac</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAuth_ClientID</name>
      <type>Main</type>
      <value>qa-sXZ3EygiKuovA7RgGjDbZ1PIRimxetbaAGczeGnW</value>
      <webElementGuid>f943c7e5-8f87-49b8-a811-cc85d3ab4e00</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sSearchURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sFrom</defaultValue>
      <description></description>
      <id>47e258ab-8dc4-42d9-ba6e-06523e37db82</id>
      <masked>false</masked>
      <name>sFrom</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sTo</defaultValue>
      <description></description>
      <id>8817cd7a-deed-4b21-8bb3-b8f4462bd17d</id>
      <masked>false</masked>
      <name>sTo</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCarrier</defaultValue>
      <description></description>
      <id>51180fad-c7be-40c7-9eaf-590dbfb5d0e7</id>
      <masked>false</masked>
      <name>sCarrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCurrencyCode</defaultValue>
      <description></description>
      <id>159ec49b-5817-4cd3-b39c-cd226c8a029f</id>
      <masked>false</masked>
      <name>sCurrencyCode</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sPTC</defaultValue>
      <description></description>
      <id>402b9d80-bb9f-4909-97bc-e93a33e69d21</id>
      <masked>false</masked>
      <name>sPTC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sPassengerCount</defaultValue>
      <description></description>
      <id>cd90160b-50f3-462d-9a46-2df96b741b98</id>
      <masked>false</masked>
      <name>sPassengerCount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>0445677f-d22c-4b36-b57d-b39cb3433b24</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sDepDate</defaultValue>
      <description></description>
      <id>11fa35b6-52d0-4cbc-bb4e-330a7d4764b0</id>
      <masked>false</masked>
      <name>sDepDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sRetDate</defaultValue>
      <description></description>
      <id>bfdea1a7-f973-4236-a975-753264211494</id>
      <masked>false</masked>
      <name>sRetDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sSearchURL</defaultValue>
      <description></description>
      <id>89bbb485-89d5-4ee1-bda7-25f814637dd0</id>
      <masked>false</masked>
      <name>sSearchURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCircleTripApt</defaultValue>
      <description></description>
      <id>082e6e7b-996e-4315-8781-a6699b771a28</id>
      <masked>false</masked>
      <name>sCircleTripApt</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCircleDate</defaultValue>
      <description></description>
      <id>be787e7f-95da-4982-a83e-a9ef2994f5e3</id>
      <masked>false</masked>
      <name>sCircleDate</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>