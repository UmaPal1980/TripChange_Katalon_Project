<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>V_Modify Offer</name>
   <tag></tag>
   <elementGuidId>1f460dc7-64c4-437c-ab9a-e676177619d7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;OfferQueryBuildFromCatalogOfferings\&quot; : {\n    \&quot;returnBrandedFaresInd\&quot; : true,\n    \&quot;reCheckInventoryInd\&quot; : true,\n    \&quot;BuildFromCatalogOfferingsRequest\&quot; : {\n      \&quot;@type\&quot; : \&quot;BuildFromCatalogOfferingsRequestAir\&quot;,\n      \&quot;CatalogOfferingsIdentifier\&quot; : {\n        \&quot;Identifier\&quot; : {\n          \&quot;value\&quot; : \&quot;${ExchSearchIdentifierValue}\&quot;\n        }\n      },\n      \&quot;CatalogOfferingIdentifier\&quot; : {\n        \&quot;Identifier\&quot; : {\n          \&quot;value\&quot; : \&quot;o0.0\&quot;\n        }\n      }\n    },\n    \&quot;FareRuleType\&quot; : \&quot;LongText\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>f2e088be-adaahdfhdatr-4</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>Locale=en_US,lastName=testuser,accessGroups=cn=512928,ou=Organization,dc=travelport,dc=com^cn=504860,ou=Organization,dc=travelport,dc=com^cn=AB6922CC-BE07-4291-B38F-AB25ED96F07E,ou=AccessGroups,dc=travelport,dc=com^cn=7C7ED10A-EEBC-4468-B499-879DE63F1B7D,ou=AccessGroups,dc=travelport,dc=com^cn=D6BBCAA7-76AE-493F-87DD-7D32F12EE31B,ou=AccessGroups,dc=travelport,dc=com,accessList=512928^504860^AB6922CC-BE07-4291-B38F-AB25ED96F07E^7C7ED10A-EEBC-4468-B499-879DE63F1B7D^D6BBCAA7-76AE-493F-87DD-7D32F12EE31B,Roles=Developer^Travel Agent,SSOSessionProperties={\\\&quot;SmAuthResults\\\&quot;:\\\&quot;AQFzABVUUzFHdGVzdHVzZXJAbWFpbC5jb21zABxOdEk3K1FKOWs1RVowbkJaYzBaNXZNUnNMYms9cwHsYXUyMUhxeEdaTW1LTXlOYUYwN3FtRTlscG5IRVdnM0hqTlYvQ0IzMzdqeHhjekJMK3FCSGpjdTNWVllaTEtzNW1wLzVSUGYxVmo3S1pVbGI2ODlDYS9kTTE5YW8rRWlHaHhUQTNKcnVlZloyR1p0NjRZQXU5Y0RNSlkzVTB4L1pLeG5mZmpQY3Zubm41Y0hvb2hDaU5rRVBvRmZQTWMwSDVjdDN5ZnlVUVZoZVVGUU43NGRaeEdxSjhtN1ZhdDlzNEZPeVZ1Wk10dVBQZ1JPWncrQ3JNc2c3TTVGNDlsWDljTyszdWRJTWhBWExkekV4MVpGeWpudHBXSjY0RERZR2JvQU0xbGlIbHI1eUlVdE1XUjJoMDJZK0Y5UXhra1ZqbStRck0wL295bUlSZzFiSG9BZGNRVm1XNE5rVkZMc1pzMGw3MDJ1V0JIRHVTOURkUG5CU0c3bSszWVlpV2gycTdIdjMzVmFMbWUvNVVPLzJiY3g2T0M1TFE3eVdWNUZOcWJEa1Y3RlFua0x6MmxiVjdIR20zMnltRzlwSFNESkRlTnYwZ2lMcm4xR0tmcmY4YXg4cko2Q0p4S1hhN3F1aFJ0cldva1h2aWN2NkpmWWduVUN3dE1VRGlXT2ZiMEs1b0JNaEdUc0o0Q3c9CAAAAXVXTpCAAjhAAnCABF+S1tAEX5LW0ARfktbS\\\&quot;},firstName=TRIPSERVICES,UID=TP98348858,Language=English,Static_Gtids=,email=TS1Gtestuser@mail.com, forwarded=for=10.107.131.31;host=ts-airbook-res-session-do-6-qab-tripservices-qa.ocp-a.zu2.nonprod.travelport.io:443;proto=https;proto-version=, xauth_travelport_accessgroup=7C7ED10A-EEBC-4468-B499-879DE63F1B7D, x-forwarded-port=443, e2etrackingid=IntegratedDemo:qand1001}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic R0ZTVkNTL0dGU1ZDQWNjb3VudDoxTGI4QiZ6IzhQ</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAuth_ClientID</name>
      <type>Main</type>
      <value>qa-sXZ3EygiKuovA7RgGjDbZ1PIRimxetbaAGczeGnW</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sModifyOfferURL}</restUrl>
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
      <id>98cf87c1-b11f-4f3e-bd0e-a9f7039b5685</id>
      <masked>false</masked>
      <name>IdentifierResWorkBench</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ExchSearchIdentifierValue</defaultValue>
      <description></description>
      <id>eca3424b-15ec-484c-8051-7d282eb8ae94</id>
      <masked>false</masked>
      <name>ExchSearchIdentifierValue</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>cd31cc45-648b-4776-b033-419e961dea79</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sModifyOfferURL</defaultValue>
      <description></description>
      <id>722ce45b-fc2f-400a-8313-6a244b816b2f</id>
      <masked>false</masked>
      <name>sModifyOfferURL</name>
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
