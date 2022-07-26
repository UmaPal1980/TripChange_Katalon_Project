<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>S_GetEligibilityRequestbyTKT</name>
   <tag></tag>
   <elementGuidId>ecfe326a-9ec4-468d-8a9f-c11609547685</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>07d9d15d-74d2-46ef-b036-7ff4f0e8e9a6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>865d3edf-a7af-4ee1-92f4-0bcc3c9a1909</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a14d721f-0c5d-4250-873b-ec16445b88d3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>2860072a-bc3d-4a4e-88bc-a9065dfe5611</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAuth_ClientID</name>
      <type>Main</type>
      <value>qa-sXZ3EygiKuovA7RgGjDbZ1PIRimxetbaAGczeGnW</value>
      <webElementGuid>87c750de-5ca1-492e-9d6a-b504e41e37b3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>Locale=en_US,lastName=testuser,accessGroups=cn=512928,ou=Organization,dc=travelport,dc=com^cn=504860,ou=Organization,dc=travelport,dc=com^cn=AB6922CC-BE07-4291-B38F-AB25ED96F07E,ou=AccessGroups,dc=travelport,dc=com^cn=7C7ED10A-EEBC-4468-B499-879DE63F1B7D,ou=AccessGroups,dc=travelport,dc=com^cn=D6BBCAA7-76AE-493F-87DD-7D32F12EE31B,ou=AccessGroups,dc=travelport,dc=com,accessList=512928^504860^AB6922CC-BE07-4291-B38F-AB25ED96F07E^7C7ED10A-EEBC-4468-B499-879DE63F1B7D^D6BBCAA7-76AE-493F-87DD-7D32F12EE31B,Roles=Developer^Travel Agent,SSOSessionProperties={\\\\\\\\\\\\\\\&quot;SmAuthResults\\\\\\\\\\\\\\\&quot;:\\\\\\\\\\\\\\\&quot;AQFzABVUUzFHdGVzdHVzZXJAbWFpbC5jb21zABxOdEk3K1FKOWs1RVowbkJaYzBaNXZNUnNMYms9cwHsYXUyMUhxeEdaTW1LTXlOYUYwN3FtRTlscG5IRVdnM0hqTlYvQ0IzMzdqeHhjekJMK3FCSGpjdTNWVllaTEtzNW1wLzVSUGYxVmo3S1pVbGI2ODlDYS9kTTE5YW8rRWlHaHhUQTNKcnVlZloyR1p0NjRZQXU5Y0RNSlkzVTB4L1pLeG5mZmpQY3Zubm41Y0hvb2hDaU5rRVBvRmZQTWMwSDVjdDN5ZnlVUVZoZVVGUU43NGRaeEdxSjhtN1ZhdDlzNEZPeVZ1Wk10dVBQZ1JPWncrQ3JNc2c3TTVGNDlsWDljTyszdWRJTWhBWExkekV4MVpGeWpudHBXSjY0RERZR2JvQU0xbGlIbHI1eUlVdE1XUjJoMDJZK0Y5UXhra1ZqbStRck0wL295bUlSZzFiSG9BZGNRVm1XNE5rVkZMc1pzMGw3MDJ1V0JIRHVTOURkUG5CU0c3bSszWVlpV2gycTdIdjMzVmFMbWUvNVVPLzJiY3g2T0M1TFE3eVdWNUZOcWJEa1Y3RlFua0x6MmxiVjdIR20zMnltRzlwSFNESkRlTnYwZ2lMcm4xR0tmcmY4YXg4cko2Q0p4S1hhN3F1aFJ0cldva1h2aWN2NkpmWWduVUN3dE1VRGlXT2ZiMEs1b0JNaEdUc0o0Q3c9CAAAAXVXTpCAAjhAAnCABF+S1tAEX5LW0ARfktbS\\\\\\\\\\\\\\\&quot;},firstName=TRIPSERVICES,UID=TP98348858,Language=English,Static_Gtids=,email=TS1Gtestuser@mail.com, forwarded=for=10.107.131.31;host=ts-airbook-res-session-do-6-qab-tripservices-qa.ocp-a.zu2.nonprod.travelport.io:443;proto=https;proto-version=, xauth_travelport_accessgroup=7C7ED10A-EEBC-4468-B499-879DE63F1B7D, x-forwarded-port=443, e2etrackingid=IntegratedDemo:qand1001}</value>
      <webElementGuid>cefbac71-5bf1-4c4f-9361-9ffb4785acfb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>NGGFRouting</name>
      <type>Main</type>
      <value>Splash=EU-GFARESTG</value>
      <webElementGuid>c1705cbe-2946-4a47-b799-1eae40c6b1bd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${sGetEligibilitybyTKTURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.PNR</defaultValue>
      <description></description>
      <id>42251950-ca64-48b5-9167-fef6cc58b98a</id>
      <masked>false</masked>
      <name>PNR</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>ba36f973-4db5-426d-83b1-cbce97b3662e</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sGetEligibilitybyTKTURL</defaultValue>
      <description></description>
      <id>7adc47fc-a342-4eaf-8461-c761ae1566ea</id>
      <masked>false</masked>
      <name>sGetEligibilitybyTKTURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sOldTicket</defaultValue>
      <description></description>
      <id>b6cbfe1b-22d8-41a9-a888-380f3fb7f782</id>
      <masked>false</masked>
      <name>sOldTicket</name>
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
